use std::mem;

pub fn primitives() {
    // i8 u8 - i16 u16 - i32 u32 - i64 u64

    // let creates a binding from value <-> memory location
    let a:u8 = 123; // immutable unsigned 8bit int (0..255)

    println!("a = {}", a);

    let mut b:i8 = -111; // mutable signed 8bit int (-127..128)
    println!("b = {}", b);

    b = 0;
    println!("b = {}", b);

    let mut c = 123456789; // inferred 32-bit signed int (i32)
    println!("c = {} ({} bytes)", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} (after mutation)", c);

    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {} ({} bytes, {}-bit os)", z, size_of_z, size_of_z * 8);

    let d:char = 'x'; // can be inferred
    println!("d = {} ({} bytes)", d, mem::size_of_val(&d));

    // f32 f64

    let e = 2.5; // inferred double-precision, 8 bytes/64 bits (f64)
    println!("e = {} ({} bytes)", e, mem::size_of_val(&e));

    let f:f32 = 2.5; // double-precision, 4 bytes/32 bits (f32)
    println!("f = {} ({} bytes)", f, mem::size_of_val(&f));

    // true false (1 byte)

    let g = false;
    println!("g = {} ({} bytes)", g, mem::size_of_val(&g));

    let h = 4 > 0; // true
    println!("h = {} ({} bytes)", h, mem::size_of_val(&h));
}
