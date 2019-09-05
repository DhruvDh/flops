use rand::prelude::*;
use packed_simd::{i32x8, f32x8};
use std::time::Instant;

/// returns an array with 8 floats that will use SIMD instructions for operations
fn new_simd_float() -> [f32; 8] {
    let mut rng = rand::thread_rng();
    let mut random_floats: [f32; 8] = [0.0; 8];

    for i in 0..8 {
        random_floats[i] = rng.gen();
    }
    
    // f32x8::from_slice_unaligned(&random_floats[0..8])
    random_floats
}

/// returns an array with 8 ints that will use SIMD instructions for operations
fn new_simd_int() -> [i32; 8] {
    let mut rng = rand::thread_rng();
    let mut random_ints: [i32; 8] = [0; 8];

    for i in 0..8 {
        random_ints[i] = rng.gen();
    }
    random_ints
}

fn main() {
    let mut a = new_simd_float(); 
    let mut b = new_simd_float(); 
    let mut c = new_simd_float(); 

    let mut x = new_simd_int(); 
    let mut y = new_simd_int(); 
    let mut z = new_simd_int(); 
    
    let now = Instant::now();
    for _ in 0..100_000_000 {
        for _ in 0..5 {
            for i in 0..8 {
                a[i] = a[i] - b[i];
                // b[i] = a[i] + b[i];
                c[i] = a[i] * b[i];

                // a[i] = c[i] - b[i];
                // b[i] = a[i] + b[i];
                // c[i] = a[i] * b[i];

                // x[i] = x[i] - y[i];
                // y[i] = x[i] + y[i];        
                // z[i] = x[i] * y[i];
            }
        }
    }

    println!("Took {:?} seconds.", now.elapsed().as_secs_f32());
    dbg!(&c); // reading the value to ensure compiler actually computes them
    dbg!(&z); // reading the value to ensure compiler actually computes them

    // let x: Vec<f32x8> = (0..100_000_000).map(|_| new_simd_float()).collect();
    // let y: Vec<i32x8> = (0..100_000_000).map(|_| new_simd_int()).collect();

    // dbg!(&x);
    // dbg!(&y);

    let mut rng = rand::thread_rng();
}
