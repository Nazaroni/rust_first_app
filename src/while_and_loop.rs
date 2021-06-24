pub fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 { continue; }    // skip 64
        println!( "x = {}", x );
    }


    let mut y = 1;

    // while true
    loop {
        y *= 2;
        println!( "y = {}", y );

        // if y is 1024
        if y == 1<<10 { break; }
    }
}