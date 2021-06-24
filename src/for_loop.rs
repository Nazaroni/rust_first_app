pub fn for_loop() {
    for x in 0..11 {
        if x == 3 || x == 5 { continue; }
        if x == 8 { break; }
        println!( "x = {}", x );
    }

    for ( index, element ) in ( 31..41 ).enumerate() {
        println!( "{}: {}", index + 1, element );
    }
}