pub fn strings() {
    // String in Rust = vector of characters

    // utf-8
    let my_string:&'static str = "Hello there!"; // &str = string slice

    // my_string = "abc" //! Error!
    // let h = my_string[ 0 ]; // Error, because you'll manipulate with bytes, not chars!

    for c in my_string.chars().rev() {
        println!( "{}", c );
    }

    if let Some( first_caracter ) = my_string.chars().nth( 0 ) {
        println!( "First letter is {}", first_caracter );
    }

    // heap
    // String - correct type to play with strings
    let mut letters = String::new();
    let mut letter = 'a' as u8;
    while letter <= ( 'z' as u8 ) {
        letters.push( letter as char );
        letters.push_str( ", " );
        letter += 1;
    }

    println!( "{}", letters );

    // $str <> String
    let u:&str = &letters;

    // concatenation: String + str
    let z = letters + "abc";
    println!( "{}", z );

    // let y = letters + &letters;
    // println!( "{}", y );

    let mut abc = String::from("Hoppla!" );
    println!( "{}", abc );

    let mut def = "Hopla shmopla".to_string();
    println!( "{}", def );

    def.remove( 0 );
    def.push_str( "!!!" );
    println!( "{}", def.replace( "opla", "Kirilomoko" ) );

}