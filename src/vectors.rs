pub fn vectors() {
    let mut my_vector = Vec::new();
    my_vector.push( 33 );
    my_vector.push( 4 );
    my_vector.push( 8 );
    println!( "my_vector = {:?}", my_vector );

    my_vector.push( 999 );
    println!( "my_vector = {:?}", my_vector );

    println!( "my_vector[ 0 ] = {}", my_vector[ 0 ] );

    // usize and isize == memory of your operational system (my case 64 bit)
    // index can only be usize
    let idx:usize = 0;

    // change value at index 0
    my_vector[ idx ] = 6464;
    println!( "my_vector[ 0 ] = {}", my_vector[ idx ] );
    println!( "my_vector = {:?}", my_vector );
    println!( "my_vector length is = {}", my_vector.len() );


    // how to safe add / change values in vectors (Option)
    let my_index = 6;
    match my_vector.get( my_index ) {
        Some( x ) => println!( "my_vector[{}] = {}", my_index, x ),
        None => println!( "error, no such element in my_vector[{}]", my_index )
    }

    // vector iteration
    println!( "\n!!!Vector Iteration!!!" );
    for x in &my_vector { println!( "{}", x ); }

    my_vector.push( 878 );
    println!( "my_vector = {:?}", my_vector );


    // REMOVE from the vector
    let last_element = my_vector.pop();
    println!( "Poped element = {:?}", last_element );
    println!( "my_vector = {:?}", my_vector );

    while let Some( x ) = my_vector.pop() {
        println!( "Last element is {:?}, my_vector = {:?}", x, my_vector );
    }

}