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
