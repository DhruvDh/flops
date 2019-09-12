use rand::prelude::*;
use rayon;
use packed_simd::{i32x8, f32x8};
use std::time::Instant;

/// returns an array with 8 floats that will use SIMD instructions for operations
fn new_simd_float() -> f32x8 {
    let mut rng = rand::thread_rng();
    let random_floats: Vec<f32> = (0..8).map(|_| rng.gen()).collect();
    
    f32x8::from_slice_unaligned(&random_floats[0..8])
}

/// returns an array with 8 ints that will use SIMD instructions for operations
fn new_simd_int() -> i32x8 {
    let mut rng = rand::thread_rng();
    let random_ints: Vec<i32> = (0..8).map(|_| rng.gen()).collect();

    i32x8::from_slice_unaligned(&random_ints[0..8])
}

macro_rules! create_variables {
    ($($x:ident,)+, $y:ident) => {
        $(
            let mut $x = $y();
        )+
    };
}

macro_rules! mul_add_them {
    ($(($x:ident, $y:ident, $z:ident)),+) => {
        $(
            $x = $x.mul_add($y, $z);
        )+
    };
}

macro_rules! debug_them {
    ($($x:ident),+) => {
        $(
            dbg!(&$x);
        )+
    }
}

fn do_math() {
    create_variables!(a, b, c, x, y, z, i, j, k, m, n, o,, new_simd_float);
    // macro that expands to let a = new_simd_float(); for each variable a, b, c...
    create_variables!(A, B, C, X, Y, Z, I, J, K, M, N, O,, new_simd_float);
    // basically initializes each variable to an array of random 8 32-bit floats that use SIMD operations 
    
    for _ in 0..(1e10 as i64) {
        mul_add_them!((a, b, c),  (x, y, z), (i, j, k), (m, n, o));
        // a macro that expands to a = a.mul_add(b, c); x = x.mul_add(y, z) and so on for each variable.
        // basically performs fused multiply add 5 times, once for each trio of variables.
        mul_add_them!((A, B, C), (X, Y, Z), (I, J, K), (M, N, O));
        // 8 fused multiply adds on arrays of 8 32-bit floats for a total of
        // 8 * 8 * 2 = 128 floating point operations in one iteration
    }
    
    debug_them!(a, x, i, m);
    debug_them!(A, X, I, M);    
    // macro that prints each variable to std_err to ensure compiler doesn't "optimize" away the computation
}

fn main() {
    let num_threads = 16;
    rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
    // builds a threadpool of 16 threads
    
    println!("In my attempt to get max flops I'll be running 8 fused multiply operations among 3 arrays of 8 32-bit floats; in a loop iterating 50 times; again in a loop iterati");
    // a time instant used for benchmarking
    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GFLOPS.", now.elapsed().as_secs_f32(), (128 * 16) as f32 / now.elapsed().as_secs_f32());
}
