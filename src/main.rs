
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

    let mut x = new_simd_float(); 
    let mut y = new_simd_float(); 
    let mut z = new_simd_float(); 

    let mut i = new_simd_float(); 
    let mut j = new_simd_float(); 
    let mut k = new_simd_float();

    let mut m = new_simd_float(); 
    let mut n = new_simd_float(); 
    let mut o = new_simd_float(); 

    // dbg!(&a); // reading the value to ensure compiler actually computes them

    let now = Instant::now();
    for _ in 0..100_000_000 {
        for _ in 0..50 {
            a = a.mul_add(b, c);
            x = x.mul_add(y, z);
            i = i.mul_add(j, k);
            o = o.mul_add(n, m);
        }
    }

    dbg!(&a); // reading the value to ensure compiler actually computes them
    dbg!(&x); // reading the value to ensure compiler actually computes them
    dbg!(&i); // reading the value to ensure compiler actually computes them
    dbg!(&o); // reading the value to ensure compiler actually computes them
    now.elapsed().as_secs_f32()
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(32).build_global().unwrap();
    
    let now = Instant::now();
    rayon::scope(|s| {
        for _ in 0..16 {
            s.spawn(|_| {
                println!("Thread {}: Took {:?} seconds.", rayon::current_thread_index().unwrap_or(0), do_stuff());
            })
        }
    });
    println!("\n\nIn all, took {:?} seconds.", now.elapsed().as_secs_f32());
}

// use rand::prelude::*;
// use packed_simd::{i32x8, f32x8};
// use std::time::Instant;

// /// returns an array with 8 floats that will use SIMD instructions for operations
// fn new_simd_float() -> [f32; 8] {
//     let mut rng = rand::thread_rng();
//     let mut random_floats: [f32; 8] = [0.0; 8];

//     for i in 0..8 {
//         random_floats[i] = rng.gen();
//     }
    
//     // f32x8::from_slice_unaligned(&random_floats[0..8])
//     random_floats
// }

// /// returns an array with 8 ints that will use SIMD instructions for operations
// fn new_simd_int() -> [i32; 8] {
//     let mut rng = rand::thread_rng();
//     let mut random_ints: [i32; 8] = [0; 8];

//     for i in 0..8 {
//         random_ints[i] = rng.gen();
//     }
//     random_ints
// }

// fn main() {
//     let mut a = new_simd_float(); 
//     let mut b = new_simd_float(); 
//     let mut c = new_simd_float(); 

//     let mut x = new_simd_int(); 
//     let mut y = new_simd_int(); 
//     let mut z = new_simd_int(); 

//     dbg!(&c); // reading the value to ensure compiler actually computes them
    
//     let now = Instant::now();
//     for _ in 0..100_000_000 {
//         for _ in 0..50 {
//             for i in 0..8 {
//                 // a[i] = a[i] - b[i];
//                 // b[i] = a[i] + b[i];
//                 // c[i] = a[i] * b[i];

//                 // a[i] = c[i] - b[i];
//                 // b[i] = a[i] + b[i];
//                 // c[i] = a[i] * b[i];

//                 a[i] = a[i].mul_add(b[i], c[i]);
//                 b[i] = b[i].mul_add(a[i], c[i]);
//                 c[i] = c[i].mul_add(b[i], a[i]);

//                 // a[i] = a[i] + (b[i] * c[i]);
//                 // b[i] = b[i] + (a[i] * c[i]);
//                 // c[i] = c[i] + (b[i] * a[i]);



//                 // x[i] = x[i] - y[i];
//                 // y[i] = x[i] + y[i];        
//                 // z[i] = x[i] * y[i];
//             }
//         }
//     }

//     println!("Took {:?} seconds.", now.elapsed().as_secs_f32());
//     dbg!(&c); // reading the value to ensure compiler actually computes them
//     dbg!(&z); // reading the value to ensure compiler actually computes them

//     // let x: Vec<f32x8> = (0..100_000_000).map(|_| new_simd_float()).collect();
//     // let y: Vec<i32x8> = (0..100_000_000).map(|_| new_simd_int()).collect();

//     // dbg!(&x);
//     // dbg!(&y);

//     let mut rng = rand::thread_rng();
// }