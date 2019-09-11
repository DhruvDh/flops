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
    ($($x:ident, $y:ident, $z:ident),+) => {
        $(
            $x = $x.mul_add($y, $z);
        )+
    };
}

fn do_stuff() -> f32 {
    create_variables!(a, b, c, x, y, z, i, j, k, m, n, o,, new_simd_float);
    create_variables!(A, B, C, D, X, Y, Z, I, J, K, M, N, O,, new_simd_float);
    // macro that expands to let a = new_simd_float() for each variable a, b, c...

    let now = Instant::now();
    for _ in 0..100_000_000 {
        for _ in 0..50 {
            mul_add_them!(a, b, c,  x, y, z, i, j, k, m, n, o);
            mul_add_them!(A, B, C, X, Y, Z, I, J, K, M, N,  O);
            // a macro that expands to a = a.mul_add(b, c); x = x.mul_add(y, z) and so on for each variable.
        }
    }
    
    dbg!(&a); // reading the value to ensure compiler actually computes them
    dbg!(&x); // reading the value to ensure compiler actually computes them
    dbg!(&i); // reading the value to ensure compiler actually computes them
    dbg!(&m); // reading the value to ensure compiler actually computes them

    dbg!(&A); // reading the value to ensure compiler actually computes them
    dbg!(&X); // reading the value to ensure compiler actually computes them
    dbg!(&I); // reading the value to ensure compiler actually computes them
    dbg!(&M); // reading the value to ensure compiler actually computes them

    

    now.elapsed().as_secs_f32()
}

fn main() {
    let threads = 1;
    rayon::ThreadPoolBuilder::new().num_threads(32).build_global().unwrap();
    
    let now = Instant::now();    
    rayon::scope(|s| {
        for _ in 0..threads {
            s.spawn(|_| {
                println!("Took {:?} seconds.", do_stuff());
            })
        };
    });

    println!("\n\nIn all, Took {:?} seconds for {} threads.", now.elapsed().as_secs_f32(), threads);
    let threads = 2;
    
    let now = Instant::now();    
    rayon::scope(|s| {
        for _ in 0..threads {
            s.spawn(|_| {
                println!("Took {:?} seconds.", do_stuff());
            })
        };
    });

    println!("\n\nIn all, Took {:?} seconds for {} threads.", now.elapsed().as_secs_f32(), threads);
    

    let threads = 4;
    
    let now = Instant::now();    
    rayon::scope(|s| {
        for _ in 0..threads {
            s.spawn(|_| {
                println!("Took {:?} seconds.", do_stuff());
            })
        };
    });

    println!("\n\nIn all, Took {:?} seconds for {} threads.", now.elapsed().as_secs_f32(), threads);    

    let threads = 8;
    
    let now = Instant::now();    
    rayon::scope(|s| {
        for _ in 0..threads {
            s.spawn(|_| {
                println!("Took {:?} seconds.", do_stuff());
            })
        };
    });

    println!("\n\nIn all, Took {:?} seconds for {} threads.", now.elapsed().as_secs_f32(), threads);

    let threads = 16;
    
    let now = Instant::now();    
    rayon::scope(|s| {
        for _ in 0..threads {
            s.spawn(|_| {
                println!("Took {:?} seconds.", do_stuff());
            })
        };
    });

    println!("\n\nIn all, Took {:?} seconds for {} threads.", now.elapsed().as_secs_f32(), threads);

    let threads = 32;
    
    let now = Instant::now();    
    rayon::scope(|s| {
        for _ in 0..threads {
            s.spawn(|_| {
                println!("Took {:?} seconds.", do_stuff());
            })
        };
    });

    println!("\n\nIn all, Took {:?} seconds for {} threads.", now.elapsed().as_secs_f32(), threads);

}
