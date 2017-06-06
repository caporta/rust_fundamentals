pub fn scope_and_shadowing() {
// block scoping

    let a = 123;

    {
        let b = 246; // not accessible outside of block
        println!("inside b = {}", b);

        println!("outside a = {}", a);

        let a = 111;
        println!("inside a = {}", a);
    }

    let a = 234; // a can be redeclared to a new mem addr
    println!("outside a = {}", a);
}
