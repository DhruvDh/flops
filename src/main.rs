
use rand::prelude::*;
use packed_simd::{f32x8};
use std::time::Instant;
use rayon::prelude::*;
use num_cpus;

/// returns an tuple representing a packed 8 lane 32-bit float
fn new_simd_float() -> f32x8 {
    let mut rng = rand::thread_rng();
    let random_floats: Vec<f32> = (0..8).map(|_| rng.gen()).collect();
    
    f32x8::from_slice_unaligned(&random_floats[0..8])
}

fn do_stuff() -> f32 {
    let mut a = new_simd_float(); 
    let mut b = new_simd_float(); 
    let mut c = new_simd_float(); 

    let now = Instant::now();
    for _ in 0..100_000_000 {
        for _ in 0..500 {
            // a = a - (b * c);
            // a.mul_add(b, c);
            // c = c - (a * b);
            // c.mul_add(a, b);
            // b = b + (b * c);
            // b.mul_add(b, c);
            a = a - b;
            c = a * b;        
        }
    }

    dbg!(&c); // reading the value to ensure compiler actually computes them
    now.elapsed().as_secs_f32()
}

fn main() {

    let now = Instant::now();
    rayon::scope(|s| {
        for _ in 0..num_cpus::get() {
            s.spawn(|t| {
                println!("Thread {}: Took {:?} seconds.", rayon::current_thread_index().unwrap_or(0), do_stuff());
            })
        }
    });
    println!("\n\nIn all, took {:?} seconds.", now.elapsed().as_secs_f32());
}