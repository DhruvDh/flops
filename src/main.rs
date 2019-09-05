use rand::prelude::*;
use std::time::Instant;
use std::thread;

/// returns an array with 8 floats that will use SIMD instructions for operations
fn new_simd_float() -> [f32; 8] {
    let mut rng = rand::thread_rng();
    let mut random_floats: [f32; 8] = [0.0; 8];

    for i in 0..8 {
        random_floats[i] = rng.gen();
    }
    
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

fn do_stuff() -> f32 {
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
            for q in 0..8 {
                a[q] = a[q].mul_add(b[q], c[q]);
                x[q] = x[q].mul_add(y[q], z[q]);
                i[q] = i[q].mul_add(j[q], k[q]);
                o[q] = o[q].mul_add(n[q], m[q]);
            }
        }
    }
    
    dbg!(&a); // reading the value to ensure compiler actually computes them
    dbg!(&x); // reading the value to ensure compiler actually computes them
    dbg!(&i); // reading the value to ensure compiler actually computes them
    dbg!(&o); // reading the value to ensure compiler actually computes them

    now.elapsed().as_secs_f32()
}

fn main() {

    let mut threads = vec![];

    let now = Instant::now();    
    for _ in 0..8 {
        threads.push(thread::spawn(|| {
            println!("Took {:?} seconds.", do_stuff());
        }));
    }

    for t in threads {
        t.join();
    }

    println!("\n\nIn all, Took {:?} seconds.", now.elapsed().as_secs_f32());

}
