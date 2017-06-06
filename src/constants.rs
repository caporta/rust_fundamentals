const MEANING_OF_LIFE:u8 = 42; // no fixed address, mandatory typing
static mut Z:i32 = 123; // use of mutable static requires unsafe fn or block

pub fn constants() {
    println!("The meaning of life = {}", MEANING_OF_LIFE);

    unsafe {
        Z = 111;
        println!("Z = {}", Z);
    }
}
