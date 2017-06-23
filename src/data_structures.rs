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

pub fn option() {
    // Option<T> returns Some(z) || None

    let x = 3.0;
    let y = 2.0;

    let result:Option<f64> =
        if y != 0.0 { Some(x/y) } else { None };

    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y),
    }

    // if let / while let
    if let Some(z) = result { println!("z = {}", z); } // destructuring result into z via pattern matching
}
