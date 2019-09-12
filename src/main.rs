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
        $a = $a * $b;
        $c = $c + $x;
        $y = $y - $i;
        $z = $z.wrapping_add($j);
    };
}

fn do_math_1() {
    let mut rng = rand::thread_rng();

    init_variables!([a, b, c, x, y, i], rng);
    init_variables!([Aa, Ab, Ac, Ax, Ay, Ai], rng);
    init_variables!([Ba, Bb, Bc, Bx, By, Bi], rng);
    init_variables!([Ca, Cb, Cc, Cx, Cy, Ci], rng);
    init_variables!([Da, Db, Dc, Dx, Dy, Di], rng);
    // macro that expands to code that initializes each variable to an array of random 8 32-bit floats that use SIMD operations 

    init_variables!((z, j, Az, Aj, Bz, Bj), rng);
    init_variables!((Cz, Cj, Dz, Dj), rng);
    // same marco also initialized to a random i32 if variables are inside () instead of []

    
    for _ in 0..(1e9 as i64) {
        math!(a, b, c, x, y, i, z, j);
        math!(Aa, Ab, Ac, Ax, Ay, Ai, Az, Aj);
        math!(Ba, Bb, Bc, Bx, By, Bi, Bz, Bj);
        math!(Ca, Cb, Cc, Cx, Cy, Ci, Cz, Cj);
        math!(Da, Db, Dc, Dx, Dy, Di, Dz, Dj);
    }
    
    debug_them!(a, c, y, z);
    // debug_them!(Aa, Ac, Ay, Az);
    // debug_them!(Ba, Bc, By, Bz);
    // debug_them!(Ca, Cc, Cy, Cz);
    // debug_them!(Da, Dc, Dy, Dz);
    // macro that prints each variable to std_err to ensure compiler doesn't "optimize" away the computation
}

fn do_math_2() {
    let mut rng = rand::thread_rng();

    init_variables!([a, b, c, x, y, i], rng);
    init_variables!([Aa, Ab, Ac, Ax, Ay, Ai], rng);
    init_variables!([Ba, Bb, Bc, Bx, By, Bi], rng);
    init_variables!([Ca, Cb, Cc, Cx, Cy, Ci], rng);
    init_variables!([Da, Db, Dc, Dx, Dy, Di], rng);
    // macro that expands to code that initializes each variable to an array of random 8 32-bit floats that use SIMD operations 

    init_variables!((z, j, Az, Aj, Bz, Bj), rng);
    init_variables!((Cz, Cj, Dz, Dj), rng);
    // same marco also initialized to a random i32 if variables are inside () instead of []

    
    for _ in 0..(1e9 as i64) {
        math!(a, b, c, x, y, i, z, j);
        math!(Aa, Ab, Ac, Ax, Ay, Ai, Az, Aj);
        math!(Ba, Bb, Bc, Bx, By, Bi, Bz, Bj);
        math!(Ca, Cb, Cc, Cx, Cy, Ci, Cz, Cj);
        math!(Da, Db, Dc, Dx, Dy, Di, Dz, Dj);
    }
    
    debug_them!(a, c, y, z);
    debug_them!(Aa, Ac, Ay, Az);
    // debug_them!(Ba, Bc, By, Bz);
    // debug_them!(Ca, Cc, Cy, Cz);
    // debug_them!(Da, Dc, Dy, Dz);
    // macro that prints each variable to std_err to ensure compiler doesn't "optimize" away the computation
}

fn do_math_3() {
    let mut rng = rand::thread_rng();

    init_variables!([a, b, c, x, y, i], rng);
    init_variables!([Aa, Ab, Ac, Ax, Ay, Ai], rng);
    init_variables!([Ba, Bb, Bc, Bx, By, Bi], rng);
    init_variables!([Ca, Cb, Cc, Cx, Cy, Ci], rng);
    init_variables!([Da, Db, Dc, Dx, Dy, Di], rng);
    // macro that expands to code that initializes each variable to an array of random 8 32-bit floats that use SIMD operations 

    init_variables!((z, j, Az, Aj, Bz, Bj), rng);
    init_variables!((Cz, Cj, Dz, Dj), rng);
    // same marco also initialized to a random i32 if variables are inside () instead of []

    
    for _ in 0..(1e9 as i64) {
        math!(a, b, c, x, y, i, z, j);
        math!(Aa, Ab, Ac, Ax, Ay, Ai, Az, Aj);
        math!(Ba, Bb, Bc, Bx, By, Bi, Bz, Bj);
        math!(Ca, Cb, Cc, Cx, Cy, Ci, Cz, Cj);
        math!(Da, Db, Dc, Dx, Dy, Di, Dz, Dj);
    }
    
    debug_them!(a, c, y, z);
    debug_them!(Aa, Ac, Ay, Az);
    debug_them!(Ba, Bc, By, Bz);
    // debug_them!(Ca, Cc, Cy, Cz);
    // debug_them!(Da, Dc, Dy, Dz);
    // macro that prints each variable to std_err to ensure compiler doesn't "optimize" away the computation
}

fn do_math_4() {
    let mut rng = rand::thread_rng();

    init_variables!([a, b, c, x, y, i], rng);
    init_variables!([Aa, Ab, Ac, Ax, Ay, Ai], rng);
    init_variables!([Ba, Bb, Bc, Bx, By, Bi], rng);
    init_variables!([Ca, Cb, Cc, Cx, Cy, Ci], rng);
    init_variables!([Da, Db, Dc, Dx, Dy, Di], rng);
    // macro that expands to code that initializes each variable to an array of random 8 32-bit floats that use SIMD operations 

    init_variables!((z, j, Az, Aj, Bz, Bj), rng);
    init_variables!((Cz, Cj, Dz, Dj), rng);
    // same marco also initialized to a random i32 if variables are inside () instead of []

    
    for _ in 0..(1e9 as i64) {
        math!(a, b, c, x, y, i, z, j);
        math!(Aa, Ab, Ac, Ax, Ay, Ai, Az, Aj);
        math!(Ba, Bb, Bc, Bx, By, Bi, Bz, Bj);
        math!(Ca, Cb, Cc, Cx, Cy, Ci, Cz, Cj);
        math!(Da, Db, Dc, Dx, Dy, Di, Dz, Dj);
    }
    
    debug_them!(a, c, y, z);
    debug_them!(Aa, Ac, Ay, Az);
    debug_them!(Ba, Bc, By, Bz);
    debug_them!(Ca, Cc, Cy, Cz);
    // debug_them!(Da, Dc, Dy, Dz);
    // macro that prints each variable to std_err to ensure compiler doesn't "optimize" away the computation
}

fn do_math_5() {
    let mut rng = rand::thread_rng();

    init_variables!([a, b, c, x, y, i], rng);
    init_variables!([Aa, Ab, Ac, Ax, Ay, Ai], rng);
    init_variables!([Ba, Bb, Bc, Bx, By, Bi], rng);
    init_variables!([Ca, Cb, Cc, Cx, Cy, Ci], rng);
    init_variables!([Da, Db, Dc, Dx, Dy, Di], rng);
    // macro that expands to code that initializes each variable to an array of random 8 32-bit floats that use SIMD operations 

    init_variables!((z, j, Az, Aj, Bz, Bj), rng);
    init_variables!((Cz, Cj, Dz, Dj), rng);
    // same marco also initialized to a random i32 if variables are inside () instead of []

    
    for _ in 0..(1e9 as i64) {
        math!(a, b, c, x, y, i, z, j);
        math!(Aa, Ab, Ac, Ax, Ay, Ai, Az, Aj);
        math!(Ba, Bb, Bc, Bx, By, Bi, Bz, Bj);
        math!(Ca, Cb, Cc, Cx, Cy, Ci, Cz, Cj);
        math!(Da, Db, Dc, Dx, Dy, Di, Dz, Dj);
    }
    
    debug_them!(a, c, y, z);
    debug_them!(Aa, Ac, Ay, Az);
    debug_them!(Ba, Bc, By, Bz);
    debug_them!(Ca, Cc, Cy, Cz);
    debug_them!(Da, Dc, Dy, Dz);
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
            s.spawn(|_| do_math_1())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * num_threads * 2) as f32 / now.elapsed().as_secs_f32());

    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math_1())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * num_threads * 1) as f32 / now.elapsed().as_secs_f32());

    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math_2())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * num_threads * 2) as f32 / now.elapsed().as_secs_f32());

    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math_3())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * num_threads * 3) as f32 / now.elapsed().as_secs_f32());

    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math_4())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * num_threads * 4) as f32 / now.elapsed().as_secs_f32());

    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math_5())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * num_threads * 5) as f32 / now.elapsed().as_secs_f32());

}
