mod primitives;
mod operators;
mod scope_and_shadowing;
mod constants;
mod stack_and_heap;
mod control_flow;
mod data_structures_1;
mod data_structures_2;

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

    println!("IF STATEMENT:\n");
    control_flow::if_statement();

    println!("\n======================\n");

    println!("WHILE AND LOOP:\n");
    control_flow::while_and_loop();

    println!("\n======================\n");

    println!("FOR LOOP:\n");
    control_flow::for_loop();

    println!("\n======================\n");

    println!("MATCH STATEMENT:\n");
    control_flow::match_statement();

    println!("\n======================\n");

    println!("STRUCTS:\n");
    data_structures_1::structures();

    println!("\n======================\n");

    println!("ENUMERATIONS:\n");
    data_structures_1::enums();

    println!("\n======================\n");

    println!("OPTION:\n");
    data_structures_1::option();

    println!("\n======================\n");

    println!("ARRAYS:\n");
    data_structures_1::arrays();

    println!("\n======================\n");

    println!("VECTORS:\n");
    data_structures_1::vectors();

    println!("\n======================\n");

    println!("SLICES:\n");
    data_structures_2::slices();

    println!("\n======================\n");

    println!("STRINGS:\n");
    data_structures_2::strings();

    println!("\n======================\n");
}
