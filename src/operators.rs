use std::f64;

pub fn operators() {

    // arithmetic

    let mut a = 2 + 3 * 4;
    println!("{}", a);

    a += 1; // does not support -- ++ incrementors
    a -= 2;
    println!("a mod 3 = {}", a % 3);

    // no exponent operator!

    let a_cubed = i32::pow(a, 3); // leverage function
    println!("a cubed = {}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    println!("b cubed = {}", b_cubed);

    let b_to_pi = f64::powf(b, f64::consts::PI);
    println!("b^pi = {}", b_to_pi);

    // bitwise

    let c = 1 | 2; // (| = OR) (& = AND) (XOR = exclusive OR) (! = negation)
    // 01 or 10 = 11 === 3_10
    println!("1|2 = {}", c);

    let two_to_ten = 1 << 10; // powers of two can be calculated bitwise
    println!("2^10 = {}", two_to_ten);

    // logical ( < > <= >= == )

    let pi_less_four = f64::consts::PI < 4.0;
    println!("pi < 4 = {}", pi_less_four);

    let x = 5;
    println!("x == 5 = {}", x == 5); // double equals
}
