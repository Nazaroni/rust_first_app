pub fn match_statement() {
    let country_code = 38;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        38 => "Ukraine",
        1..=999 => "unknown",   // ... meaning include first and last values (both)
        _ => "invalid"          // _ meaning - the rest...
    };

    println!( "the country with code {} is {}", country_code, country );
}