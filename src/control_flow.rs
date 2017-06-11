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
