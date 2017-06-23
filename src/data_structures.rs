use std::mem;

// ==================== STRUCT ==================== //

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

pub fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("Point p is at {:?}", (p.x, p.y));

    let p2 = Point { x: 5.0, y: 10.0 };

    let myLine = Line { start: p, end: p2 };
    println!("my line extends from {:?} to {:?}", (myLine.start.x, myLine.start.y), (myLine.end.x, myLine.end.y));
}

// ==================== ENUM ==================== //

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8), // tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8}, // struct
}

pub fn enums() {
    // let c:Color = Color::RGBColor(5,0,0);
    let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};

    match c {
        // exhaustive-patterns required (or _ for catch-all case)
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RGBColor(0,0,0) => println!("blk"),
        Color::RGBColor(r,g,b) => println!("rgb({},{},{})", r, g, b),
        Color::CmykColor{cyan:_, magenta: _, yellow: _, black: _} => println!("blk"),
        _ => ()
    }
}

// ==================== OPTION<T> ==================== //

pub fn option() {
    // Option<T> returns Some(z) || None

    let x = 3.0;
    let y = 2.0;

    let result:Option<f64> =
        if y != 0.0 { Some(x/y) } else { None };

    println!("{:?}", result); // debug printing

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y),
    }

    // if let
    if let Some(z) = result { println!("z = {}", z); } // destructuring result into z via pattern matching
}

// ==================== ARRAYS ==================== //

pub fn arrays() {
    let mut a:[i32;5] = [1,2,3,4,5];
    println!("a has {} elements, the first of which is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a[0] = {}", a[0]);

    println!("{:?}", a); // debug printing

    if a != [1,2,3,4,5] { println!("does not match"); }

    let b = [1u16;10];
    for i in 0..b.len() { println!("{}", b[i]); }
    println!("b takes up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3];2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("matrix is: {:?}", mtx);

    // print out the matrix's diagonal
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

// ==================== VECTORS ==================== //

pub fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("{:?}", a);

    // usize isize

    // let idx:i32 = 0;
    let idx:usize = 0; // if out of bounds, thread will panic

    // signed vals cannot be used as indexes
    // - memory addresses can't be negative
    // - wouldn't use 32 bits for memaddr on 64 bit machien

    println!("a[0] = {}", a[idx]);

    // Option<T> for .get() method
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error: no such element"),
    }

    for x in &a { println!("{}", x); }

    a.push(44);
    println!("{:?}", a);

    let last_elem = a.pop();
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    // while let

    while let Some(x) = a.pop() { // pattern matching
        println!("{}", x);
    }
}
