mod primitives;
mod operators;
mod scope_and_shadowing;
mod constants;

fn main() {
    println!("======================\n");
    println!("PRIMITIVES:\n");
    primitives::primitives();
    println!("\n======================\n");
    println!("OPERATORS:\n");
    operators::operators();
    println!("\n======================\n");
    println!("SCOPE_AND_SHADOWING:\n");
    scope_and_shadowing::scope_and_shadowing();
    println!("\n======================\n");
    println!("CONSTANTS:\n");
    constants::constants();
    println!("\n======================\n");
}
