pub fn tuples() {

    fn sum_and_product( x:i32, y:i32) -> ( i32, i32 ) {
        ( x + y, x * y )
    }

    let x = 3;
    let y = 4;
    let sp = sum_and_product( x, y );

    println!( "sp = {:?}", sp );
    println!( "{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1 );

    // Destructuring
    let ( a, b ) = sp;
    println!( "a = {}, b = {}", a, b );

    let sp2 = sum_and_product( 4, 7 );
    let combined = ( sp, sp2 );
    println!( "{:?}", combined );

    println!( "The last element = {}", (combined.1).1 );

    // https://docs.microsoft.com/en-us/learn/modules/rust-create-program/4-tuples-structs
    // Tuple of length 3
    let tuple_e = ('E', 1234567890i32, true);
    println!( "tuple_e 0: {}", tuple_e.0 );
    println!( "tuple_e 1: {}", tuple_e.1 );
    println!( "tuple_e 2: {}", tuple_e.2 );

}
