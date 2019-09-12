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

fn do_math_6() {
    let mut rng = rand::thread_rng();

    init_variables!([a, b, c, x, y, i], rng);
    init_variables!([Aa, Ab, Ac, Ax, Ay, Ai], rng);
    init_variables!([Ba, Bb, Bc, Bx, By, Bi], rng);
    init_variables!([Ca, Cb, Cc, Cx, Cy, Ci], rng);
    init_variables!([Da, Db, Dc, Dx, Dy, Di], rng);
    init_variables!([Ea, Eb, Ec, Ex, Ey, Ei], rng);
    
    // macro that expands to code that initializes each variable to an array of random 8 32-bit floats that use SIMD operations 

    init_variables!((z, j, Az, Aj, Bz, Bj), rng);
    init_variables!((Cz, Cj, Dz, Dj), rng);
    init_variables!((Ez, Ej), rng);

    // same marco also initialized to a random i32 if variables are inside () instead of []

    
    for _ in 0..(1e9 as i64) {
        math!(a, b, c, x, y, i, z, j);
        math!(Aa, Ab, Ac, Ax, Ay, Ai, Az, Aj);
        math!(Ba, Bb, Bc, Bx, By, Bi, Bz, Bj);
        math!(Ca, Cb, Cc, Cx, Cy, Ci, Cz, Cj);
        math!(Da, Db, Dc, Dx, Dy, Di, Dz, Dj);
        math!(Ea, Eb, Ec, Ex, Ey, Ei, Ez, Ej);
    }
    
    debug_them!(a, c, y, z);
    debug_them!(Aa, Ac, Ay, Az);
    debug_them!(Ba, Bc, By, Bz);
    debug_them!(Ca, Cc, Cy, Cz);
    debug_them!(Da, Dc, Dy, Dz);
    debug_them!(Ea, Ec, Ey, Ez);

    // macro that prints each variable to std_err to ensure compiler doesn't "optimize" away the computation
}

fn do_math_7() {
    let mut rng = rand::thread_rng();

    init_variables!([a, b, c, x, y, i], rng);
    init_variables!([Aa, Ab, Ac, Ax, Ay, Ai], rng);
    init_variables!([Ba, Bb, Bc, Bx, By, Bi], rng);
    init_variables!([Ca, Cb, Cc, Cx, Cy, Ci], rng);
    init_variables!([Da, Db, Dc, Dx, Dy, Di], rng);
    init_variables!([Ea, Eb, Ec, Ex, Ey, Ei], rng);
    init_variables!([Fa, Fb, Fc, Fx, Fy, Fi], rng);
 
    // macro that expands to code that initializes each variable to an array of random 8 32-bit floats that use SIMD operations 

    init_variables!((z, j, Az, Aj, Bz, Bj), rng);
    init_variables!((Cz, Cj, Dz, Dj), rng);
    init_variables!((Ez, Ej, Fz, Fj), rng);
    
    // same marco also initialized to a random i32 if variables are inside () instead of []

    
    for _ in 0..(1e9 as i64) {
        math!(a, b, c, x, y, i, z, j);
        math!(Aa, Ab, Ac, Ax, Ay, Ai, Az, Aj);
        math!(Ba, Bb, Bc, Bx, By, Bi, Bz, Bj);
        math!(Ca, Cb, Cc, Cx, Cy, Ci, Cz, Cj);
        math!(Da, Db, Dc, Dx, Dy, Di, Dz, Dj);
        math!(Ea, Eb, Ec, Ex, Ey, Ei, Ez, Ej);
        math!(Fa, Fb, Fc, Fx, Fy, Fi, Fz, Fj);
    }
    
    debug_them!(a, c, y, z);
    debug_them!(Aa, Ac, Ay, Az);
    debug_them!(Ba, Bc, By, Bz);
    debug_them!(Ca, Cc, Cy, Cz);
    debug_them!(Da, Dc, Dy, Dz);
    debug_them!(Ea, Ec, Ey, Ez);
    debug_them!(Fa, Fc, Fy, Fz);

    // macro that prints each variable to std_err to ensure compiler doesn't "optimize" away the computation
}

fn do_math_8() {
    let mut rng = rand::thread_rng();

    init_variables!([a, b, c, x, y, i], rng);
    init_variables!([Aa, Ab, Ac, Ax, Ay, Ai], rng);
    init_variables!([Ba, Bb, Bc, Bx, By, Bi], rng);
    init_variables!([Ca, Cb, Cc, Cx, Cy, Ci], rng);
    init_variables!([Da, Db, Dc, Dx, Dy, Di], rng);
    init_variables!([Ea, Eb, Ec, Ex, Ey, Ei], rng);
    init_variables!([Fa, Fb, Fc, Fx, Fy, Fi], rng);
    init_variables!([Ga, Gb, Gc, Gx, Gy, Gi], rng);
 
    init_variables!([AAa, AAb, AAc, AAx, AAy, AAi], rng);
    init_variables!([ABa, ABb, ABc, ABx, ABy, ABi], rng);
    init_variables!([ACa, ACb, ACc, ACx, ACy, ACi], rng);
    init_variables!([ADa, ADb, ADc, ADx, ADy, ADi], rng);
    init_variables!([AEa, AEb, AEc, AEx, AEy, AEi], rng);
    init_variables!([AFa, AFb, AFc, AFx, AFy, AFi], rng);
    init_variables!([AGa, AGb, AGc, AGx, AGy, AGi], rng);
    init_variables!([AHa, AHb, AHc, AHx, AHy, AHi], rng);

    // macro that expands to code that initializes each variable to an array of random 8 32-bit floats that use SIMD operations 


    init_variables!((z, j, Az, Aj, Bz, Bj), rng);
    init_variables!((Cz, Cj, Dz, Dj), rng);
    init_variables!((Ez, Ej, Fz, Fj, Gz, Gj), rng);

    init_variables!((AAz, AAj, ABz, ABj), rng);
    init_variables!((ACz, ACj, ADz, ADj), rng);
    init_variables!((AEz, AEj, AFz, AFj, AGz, AGj, AHz, AHj), rng);

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
    
        math!(AAa, AAb, AAc, AAx, AAy, AAi, AAz, AAj);
        math!(ABa, ABb, ABc, ABx, ABy, ABi, ABz, ABj);
        math!(ACa, ACb, ACc, ACx, ACy, ACi, ACz, ACj);
        math!(ADa, ADb, ADc, ADx, ADy, ADi, ADz, ADj);
        math!(AEa, AEb, AEc, AEx, AEy, AEi, AEz, AEj);
        math!(AFa, AFb, AFc, AFx, AFy, AFi, AFz, AFj);
        math!(AGa, AGb, AGc, AGx, AGy, AGi, AGz, AGj);
        math!(AHa, AHb, AHc, AHx, AHy, AHi, AHz, AHj);    
    }
    
    debug_them!(a, c, y, z);
    debug_them!(Aa, Ac, Ay, Az);
    debug_them!(Ba, Bc, By, Bz);
    debug_them!(Ca, Cc, Cy, Cz);
    debug_them!(Da, Dc, Dy, Dz);
    debug_them!(Ea, Ec, Ey, Ez);
    debug_them!(Fa, Fc, Fy, Fz);
    debug_them!(Ga, Gc, Gy, Gz);


    debug_them!(AAa, AAc, AAy, AAz);
    debug_them!(ABa, ABc, ABy, ABz);
    debug_them!(ACa, ACc, ACy, ACz);
    debug_them!(ADa, ADc, ADy, ADz);    
    debug_them!(AEa, AEc, AEy, AEz);
    debug_them!(AFa, AFc, AFy, AFz);
    debug_them!(AGa, AGc, AGy, AGz);
    debug_them!(AHa, AHc, AHy, AHz);
    // macro that prints each variable to std_err to ensure compiler doesn't "optimize" away the computation
}

fn do_math_9() {
    let mut rng = rand::thread_rng();

    init_variables!([a, b, c, x, y, i], rng);
    init_variables!([Aa, Ab, Ac, Ax, Ay, Ai], rng);
    init_variables!([Ba, Bb, Bc, Bx, By, Bi], rng);
    init_variables!([Ca, Cb, Cc, Cx, Cy, Ci], rng);
    init_variables!([Da, Db, Dc, Dx, Dy, Di], rng);
    init_variables!([Ea, Eb, Ec, Ex, Ey, Ei], rng);
    init_variables!([Fa, Fb, Fc, Fx, Fy, Fi], rng);
    init_variables!([Ga, Gb, Gc, Gx, Gy, Gi], rng);
    init_variables!([Ha, Hb, Hc, Hx, Hy, Hi], rng);
 
    // macro that expands to code that initializes each variable to an array of random 8 32-bit floats that use SIMD operations 

    init_variables!((z, j, Az, Aj, Bz, Bj), rng);
    init_variables!((Cz, Cj, Dz, Dj), rng);
    init_variables!((Ez, Ej, Fz, Fj, Gz, Gj, Hz, Hj), rng);

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
        math!(Ha, Hb, Hc, Hx, Hy, Hi, Hz, Hj);    
    }
    
    debug_them!(a, c, y, z);
    debug_them!(Aa, Ac, Ay, Az);
    debug_them!(Ba, Bc, By, Bz);
    debug_them!(Ca, Cc, Cy, Cz);
    debug_them!(Da, Dc, Dy, Dz);    
    debug_them!(Ea, Ec, Ey, Ez);
    debug_them!(Fa, Fc, Fy, Fz);
    debug_them!(Ga, Gc, Gy, Gz);
    debug_them!(Ha, Hc, Hy, Hz);
  
    // macro that prints each variable to std_err to ensure compiler doesn't "optimize" away the computation
}

fn do_math_10() {
    let mut rng = rand::thread_rng();

    init_variables!([a, b, c, x, y, i], rng);
    init_variables!([Aa, Ab, Ac, Ax, Ay, Ai], rng);
    init_variables!([Ba, Bb, Bc, Bx, By, Bi], rng);
    init_variables!([Ca, Cb, Cc, Cx, Cy, Ci], rng);
    init_variables!([Da, Db, Dc, Dx, Dy, Di], rng);
    init_variables!([Ea, Eb, Ec, Ex, Ey, Ei], rng);
    init_variables!([Fa, Fb, Fc, Fx, Fy, Fi], rng);
    init_variables!([Ga, Gb, Gc, Gx, Gy, Gi], rng);
    init_variables!([Ha, Hb, Hc, Hx, Hy, Hi], rng);
    init_variables!([Ia, Ib, Ic, Ix, Iy, Ii], rng);
 
    // macro that expands to code that initializes each variable to an array of random 8 32-bit floats that use SIMD operations 

    init_variables!((z, j, Az, Aj, Bz, Bj), rng);
    init_variables!((Cz, Cj, Dz, Dj), rng);
    init_variables!((Ez, Ej, Fz, Fj, Gz, Gj, Hz, Hj), rng);
    init_variables!((Iz, Ij), rng);

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
        math!(Ha, Hb, Hc, Hx, Hy, Hi, Hz, Hj);
        math!(Ia, Ib, Ic, Ix, Iy, Ii, Iz, Ij);
    }
    
    debug_them!(a, c, y, z);
    debug_them!(Aa, Ac, Ay, Az);
    debug_them!(Ba, Bc, By, Bz);
    debug_them!(Ca, Cc, Cy, Cz);
    debug_them!(Da, Dc, Dy, Dz);
    debug_them!(Ea, Ec, Ey, Ez);
    debug_them!(Fa, Fc, Fy, Fz);
    debug_them!(Ga, Gc, Gy, Gz);
    debug_them!(Ha, Hc, Hy, Hz);
    debug_them!(Ia, Ic, Iy, Iz);

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

    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math_6())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * num_threads * 6) as f32 / now.elapsed().as_secs_f32());

    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math_7())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * num_threads * 7) as f32 / now.elapsed().as_secs_f32());

    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math_8())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * num_threads * 16) as f32 / now.elapsed().as_secs_f32());


    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math_9())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * num_threads * 9) as f32 / now.elapsed().as_secs_f32());


    let now = Instant::now();    

    // runs the function `do_math()` 16 times on 16 threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|_| do_math_10())
        };
    });

    println!("In all, Took {:?} seconds. {:?} GIOPS.", now.elapsed().as_secs_f32(), (25 * num_threads * 10) as f32 / now.elapsed().as_secs_f32());

}
