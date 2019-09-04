use rand::prelude::*;
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
            a = a - b;
            b = a + b;
            c = a * b;

            a = c - b;
            b = a + b;
            c = a * b;

            x = x - y;
            y = x + y;        
            z = x * y;
        }
    }

    println!("Took {:?} seconds.", now.elapsed().as_secs_f32());
    dbg!(&c); // reading the value to ensure compiler actually computes them
    dbg!(&z); // reading the value to ensure compiler actually computes them

    // let x: Vec<f32x8> = (0..100_000_000).map(|_| new_simd_float()).collect();
    // let y: Vec<i32x8> = (0..100_000_000).map(|_| new_simd_int()).collect();

    // dbg!(&x);
    // dbg!(&y);
}
