pub fn slices() {

    // Slices can modify Array in Rust
    fn use_slice( slice: &mut[ i32 ] ) {
        println!( "First element = {}, len = {}", slice[ 0 ], slice.len() );
        slice[ 0 ] = 43221;
    }

    let mut data = [ 1, 2, 3, 4, 5 ];
    println!( "{:?}", data );

    use_slice( &mut data[ 1..4 ] );
    use_slice( &mut data );     // copy entire array
    println!( "{:?}", data );
}