use rand::prelude::*;
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

    let mut x = new_simd_float(); 
    let mut y = new_simd_float(); 
    let mut z = new_simd_float(); 

    let mut i = new_simd_float(); 
    let mut j = new_simd_float(); 
    let mut k = new_simd_float();

    let mut m = new_simd_float(); 
    let mut n = new_simd_float(); 
    let mut o = new_simd_float();
    
    let now = Instant::now();
    for _ in 0..100_000_000 {
        for _ in 0..50 {
            for i in 0..8 {
                a[i] = a[i].mul_add(b[i], c[i]);
                x[i] = x[i].mul_add(y[i], z[i]);
                i[i] = i[i].mul_add(j[i], k[i]);
                o[i] = o[i].mul_add(n[i], m[i]);
            }
        }
    }

    println!("Took {:?} seconds.", now.elapsed().as_secs_f32());
    dbg!(&a); // reading the value to ensure compiler actually computes them
    dbg!(&x); // reading the value to ensure compiler actually computes them
    dbg!(&i); // reading the value to ensure compiler actually computes them
    dbg!(&o); // reading the value to ensure compiler actually computes them
}
