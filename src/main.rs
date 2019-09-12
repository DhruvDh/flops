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

macro_rules! debug_them {
    ($($x:ident),+) => {
        $(
            dbg!(&$x);
        )+
    }
}

fn do_math() {
    create_variables!(a, b, c, x, y, i,, new_simd_int);
    let mut rng = rand::thread_rng();
    let mut z: i32 = rng.gen();
    let j: i32 = rng.gen();

    // macro that expands to let a = new_simd_float(); for each variable a, b, c...
    // basically initializes each variable to an array of random 8 32-bit floats that use SIMD operations 
    
    for _ in 0..(1e10 as i64) {
        a = a * b;
        c = c + x;
        y = y - i;
        z = z.wrapping_add(j);
    }
    
    debug_them!(a, c, y);
    // macro that prints each variable to std_err to ensure compiler doesn't "optimize" away the computation
}

fn main() {
    let num_threads = 16;
    rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
    // builds a threadpool of 16 threads
 
    // a time instant used for benchmarking
    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * 10 * num_threads) as f32 / now.elapsed().as_secs_f32());
}
