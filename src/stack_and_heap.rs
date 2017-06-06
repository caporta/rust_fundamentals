#[allow(dead_code)]
#[allow(unused_variable)]
use std::mem;

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{ x: 0.5, y: 1.0 }
}

pub fn stack_and_heap() {
    let x = 5; // i32 stored on the stack
    fn inc(x:i32) { println!("x + 1 = {}", x + 1) }
    inc(32); // value of 33 stored on the stack

    let y = Box::new(10);
    println!("y = {}", *y); // asterisk follows the mem addr to value

    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 ({} bytes)", mem::size_of_val(&p1));
    println!("p2 ({} bytes)", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("p3 = {}", p3.x);
    println!("p3 ({} bytes)", mem::size_of_val(&p3));
}
