mod primitives;
mod operators;
mod scope_and_shadowing;
mod constants;
mod stack_and_heap;

fn main() {
    println!("======================\n");
    println!("PRIMITIVES:\n");
    primitives::primitives();
    println!("\n======================\n");
    println!("OPERATORS:\n");
    operators::operators();
    println!("\n======================\n");
    println!("SCOPE AND SHADOWING:\n");
    scope_and_shadowing::scope_and_shadowing();
    println!("\n======================\n");
    println!("CONSTANTS:\n");
    constants::constants();
    println!("\n======================\n");
    println!("STACK AND HEAP:\n");
    stack_and_heap::stack_and_heap();
    println!("\n======================\n");
}
