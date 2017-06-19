pub fn if_statement() {
    let temp = 45;

    if temp > 30 {
        println!("really hot");
    } else if temp < 10 {
        println!("really cold");
    } else {
        println!("lukewarm")
    }

    let day = if temp > 20 { "sunny" } else if temp < 5 { "snowy" } else { "cloudy" };
    println!("{}", day);

    println!("is it {} out?", if temp > 20 { "hot" } else { "ok" });

    println!("it is {}",
        if temp > 20 {
            if temp > 30 { "very hot" } else { "hot" }
        } else { "ok" })
}

pub fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 { continue; } // resume execution from top without running subsequent code ('skip this')
        println!("x = {}", x);
    }

    let mut y = 1;
    loop {  // equivalent to `while true`
        y *= 2;

        println!("y = {}", y);

        if y == 1 << 10 { break; };  // jumps out of loop at 2**10
    }
}

pub fn for_loop() {
    for x in 1..11 { // exclusive upper bound
        if x == 3 { continue; }

        if x == 8 { break; }

        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

pub fn match_statement() { // pattern matching
    let country_code = 44; // 1..1000

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1...999 => "unknown", // inclusive range notation (opposite in most langs)
        _ => "invalid"
    };

    println!("The country with code {} is {}", country_code, country);
}
