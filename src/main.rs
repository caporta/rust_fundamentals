mod primitives;
mod operators;
mod scope_and_shadowing;

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
}
