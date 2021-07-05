pub fn scope_and_shadowing() {
    let a = 12;
    let a = 123; // override previous "a"... omg it's possible... fuck! :((((

    {
        let b = 456;
        let a = 987;
        println!( "inside, b = {}", b );
        println!( "inside, a = {}", a );
    }

    println!( "outside, a = {}", a );
}