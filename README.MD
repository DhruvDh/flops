# Assignment 1: Benchmarking Flops and Iops

- [Assignment 1: Benchmarking Flops and Iops](#assignment-1-benchmarking-flops-and-iops)
  - [I. Model](#i-model)
  - [II. Benchmarking](#ii-benchmarking)
    - [Floating point operations (Result: 1345.6177 GFLOPs)](#floating-point-operations-result-13456177-gflops)
      - [Code](#code)
    - [Integer Operations (Result: 613.46686 GIOPs)](#integer-operations-result-61346686-giops)
      - [Code](#code-1)

## I. Model

1. What is a compute node of mamba composed of? What kind of processor it has? Which architecture is the processor built on? (For instance, some core i7 are based on the ivy-bridge architecture, some on the haswell architecture, ...)

Dual Intel Xeon 3.2 GHz 8-core processors – E5-2667 v3 or v4 - Haswell (v3) and Broadwell (v4)

2. What is the maximum number of floating point operations this machine can perform per second?

Both Haswell and Broadwell can do -

- 16 64-bit FLOPs/cycle (two 4-wide FMA instructions)
- 32 32-bit FLOPs/cycle (two 8-wide FMA instructions)

Base frequency is 3.2 GHz for v3 and v4 also, so 3.2 x 10^9 cycles per second, and thusly either 16 x 3.2 x 10^9 64-bit FLOPs per cycle per core or 32 x 3.2 x 10^9 32-bit FLOPs per cycle per core; which comes to 51.2 GFLOPs or 102.4 FLOPs per cycle per core.

In all, accounting for the 8 cores brings the number to 409.6 GFLOPs/sec (64-bit) or 819.2 GFLOPs/sec (32-bit) at base frequency, assuming perfect thermals and no thermal throttling. Trying to account for thermals is pointless at best on account of so many variables involved.

3. What is the maximum number of integer operations this machine can perform per second?

Both Haswell and Broadwell can do -

- 13 64-bit IOPs/cycle (three 4-wide vector ALU instructions + 1 integer ALU instruction)
- 25 32-bit IOPs/cycle (three 8-wide vector ALU instructions + 1 integer ALU instruction)

In all, accounting for the 8 cores brings the number to 332.8 GIOPs/sec (64-bit) or 665.6‬ GIOPs/sec (32-bit) at base frequency, assuming perfect thermals and no thermal throttling. Trying to account for thermals is pointless at best on account of so many variables involved.

## II. Benchmarking

### Floating point operations (Result: 1345.6177 GFLOPs)

To achieve max flops, in a loop that iterations 10 billion times, I ran 8 FMA operations on 32-bit 8 lane SIMD arrays, coming to (`8 lanes * 8 FMAs * 2 flop per FMA`) `128` flops per iteration; one such loop per thread, for 16 threads, coming to `16 * 128 * 10^10` ( = `2.048*10^14`) flops in total.

And `2.048e14` flops took `15.219776` seconds on MAMBA, coming to `1345.6177` GFLOPs. 

#### Code

Sorry for using a bunch of marcros, but it helps rapid experimentation.

```rust
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
            $x = $x.mul_add($y, $y);
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
 
    // a time instant used for benchmarking
    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GFLOPS.", now.elapsed().as_secs_f32(), (128 * 10 * 16 ) as f32 / now.elapsed().as_secs_f32());
}

```

### Integer Operations (Result: 613.46686 GIOPs)

Did `200` integer opertions in one loop iteration, iterating 1 billion times, coming to `2e13` integer ops in total per thread, and `3.2e14` across all threads.

Took `5.2162557` seconds, coming to `613.46686` GIOPs.

#### Code

```rust
use rand::prelude::*;
use rayon;
use packed_simd::{i32x8, f32x8};
use std::time::Instant;

macro_rules! init_variables {
    ([$($x:ident),+], $rng:ident) => {
        $(
            let mut $x = {
                let random_ints: Vec<i32> = (0..8).map(|_| $rng.gen()).collect();
                i32x8::from_slice_unaligned(&random_ints[0..8])
            };
        )+
    };
    (($($x:ident),+), $rng:ident) => {
        $(
            let mut $x: i32 = $rng.gen();
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

macro_rules! math {
    ($a:ident, $b:ident, $c:ident, $x:ident, $y:ident, $i:ident, $z:ident, $j:ident) => {
        $a = $a * $a;
        $c = $c + $x;
        $y = $y - $i;
        $z = $z - $j;
    };
}

fn do_math() {
    let mut rng = rand::thread_rng();

    init_variables!([a, b, c, x, y, i], rng);
    init_variables!([Aa, Ab, Ac, Ax, Ay, Ai], rng);
    init_variables!([Ba, Bb, Bc, Bx, By, Bi], rng);
    init_variables!([Ca, Cb, Cc, Cx, Cy, Ci], rng);
    init_variables!([Da, Db, Dc, Dx, Dy, Di], rng);
    init_variables!([Ea, Eb, Ec, Ex, Ey, Ei], rng);
    init_variables!([Fa, Fb, Fc, Fx, Fy, Fi], rng);
    init_variables!([Ga, Gb, Gc, Gx, Gy, Gi], rng);
    // macro that expands to code that initializes each variable to an array of random 8 32-bit floats that use SIMD operations 


    init_variables!((z, j, Az, Aj, Bz, Bj), rng);
    init_variables!((Cz, Cj, Dz, Dj), rng);
    init_variables!((Ez, Ej, Fz, Fj, Gz, Gj), rng);
    // same marco also initialized to a random i32 if variables are inside () instead of []

    
    for _ in 0..(1e9 as i64) {
        math!(a, b, c, x, y, i, z, j);
        math!(Aa, Ab, Ac, Ax, Ay, Ai, Az, Aj);
        math!(Ba, Bb, Bc, Bx, By, Bi, Bz, Bj);
        math!(Ca, Cb, Cc, Cx, Cy, Ci, Cz, Cj);
        math!(Da, Db, Dc, Dx, Dy, Di, Dz, Dj);
        math!(Ea, Eb, Ec, Ex, Ey, Ei, Ez, Ej);
        math!(Fa, Fb, Fc, Fx, Fy, Fi, Fz, Fj);
        math!(Ga, Gb, Gc, Gx, Gy, Gi, Gz, Gj);  
        // does 25 integer ops per math!() macro call among all variables 
        // one vect mul, one vect add, one vect sub, and a single i32 addtion
        // 8 such calls amounting too 200 integer OPs per iteration 
    }
    
    debug_them!(a, c, y, z);
    debug_them!(Aa, Ac, Ay, Az);
    debug_them!(Ba, Bc, By, Bz);
    debug_them!(Ca, Cc, Cy, Cz);
    debug_them!(Da, Dc, Dy, Dz);
    debug_them!(Ea, Ec, Ey, Ez);
    debug_them!(Fa, Fc, Fy, Fz);
    debug_them!(Ga, Gc, Gy, Gz);
    // macro that prints each variable to std_err to ensure compiler doesn't "optimize" away the computation
}

fn main() {
    let num_threads = 16;
    rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
    // builds a threadpool of 16 threads
 
    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * num_threads * 8) as f32 / now.elapsed().as_secs_f32());
}
```‬