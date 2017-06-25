struct Point<T> {
    x: T,
    y: T,
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

pub fn generics() {
    let a:Point<f64> = Point { x: 0.0, y: 5.0 };
    let b = Point { x: 4.2, y: 5.0 };

    println!("a = {:?}, b = {:?}", (a.x, a.y), (b.x, a.y));
    
    let myLine = Line { start: a, end: b };
}
