pub fn if_statement() {
    let temp = 34;

    if temp > 30 {
        println!( "really hot outside" );
    }
    else if temp < 10 {
        println!( "really cold outside" );
    }
    else {
        println!( "temperature is OK!" );
    }

    // cool rust if short version
    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!( "today is {}", day );

    // inside in print macros
    println!( "is it {}", if temp > 20 { "hot" } else if temp < 10 { "cold" } else { "OK" });

    // if inside if
    println!( "it is {}",
        if temp > 20 {
                if temp > 30 { "very hot" } else { "hot" }
        }
        else if temp < 10 { "cold" } else { "OK!" } );
}