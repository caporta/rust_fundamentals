// ==================== SLICES ==================== //

fn use_slice(slice: &mut[i32]) { // &[i32] -> borrow a partition of [i32]
    println!("first elem: {}, length: {}", slice[0], slice.len());
    slice[0] = 4321;
}

pub fn slices() {
    let mut data = [1,2,3,4,5];

    //use_slice(&mut data[1..4]);
    use_slice(&mut data);
    println!("{:?}", data);
}

// ==================== STRINGS/&STRs ==================== //

pub fn strings() {
    let s:&'static str = "Hello there!"; //vector of utf-8 characters, &str = "string slice"

    //s = "abc"; s[0] = "h"; let h = s[0]; -> immutable; any mods to &str will throw error
    println!("{}", s);

    for c in s.chars().rev() { //.chars() returns sequence of characters
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) { println!("first letter is {}", first_char); }

    // "String" is a heap allocated construct (utf-8 sequence); more flexible
    let mut letters = String::new();

    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    // &str <> String
    let u:&str = &letters; // d-ref conversions

    // concatenation
    // String + &str
    //let y = letters + "abc";

    // String + String requires d-ref conversion of 2nd operand
    //let z = letters + &letters;

    // String from &str
    let mut abc = String::from("hello world");
    let mut bcd = "hello world".to_string();

    abc.push_str("!!!");
    bcd.remove(0);
    println!("abc = {}, bcd = {}", abc, bcd.replace("ello", "goodbye"));
}

// ==================== TUPLES ==================== //

// member elements can be of different types (unlike arrays)

fn sum_and_product(x:i32, y:i32) -> (i32, i32) { //tuple
    (x + y, x * y)
}

pub fn tuples() {
    let x = 3;
    let y = 4;

    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);

    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1); // dot notation accesses positions of tuples

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2,); //tuple of tuples
    println!("{:?}", combined);
    println!("last element = {}", (combined.1).1); //use parens to evaluate top level tuple first

    let ((c, d), (e, f)) = combined;
    println!("{}, {}, {}, {}", c,d,e,f);

    let meaning = (42,); // 1-elem tuple requires trailing comma. otherwise it's simply an int wrapped in noop parens
    println!("{:?}", meaning);
}

// ==================== SUMMARY ==================== //

// - A slice is a part of an array; its size is not known in advance
// - Rust has two types of strings: &'static str and String
// - Tuples can be used to store several values of any distinct types; destructuring
// - Match is extremely powerful
// - Generics let you write type-agnostic containers and functions
