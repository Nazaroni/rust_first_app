use std::mem;

pub fn arrays() {
    // in rust array should be fixed in length, I should know that I'll use only 5 elements in array
    let mut my_array:[ i32; 5 ] = [ 1, 2, 3, 4, 5 ];
    println!( "Init array: {:?}", my_array );
    println!( "my_array has {} elements, first is {}", my_array.len(), my_array[ 0 ] );

    my_array[ 0 ] = 555;
    println!( "After update my_array[ 0 ] now has = {}!", my_array[ 0 ] );

    // print the whole array
    println!( "{:?}", my_array );

    if my_array != [ 1, 2, 3, 4, 5 ] {
        println!( "doesn't match" );
    }

    let fill_by_one = [ 1u8; 10 ];
    for i in 0..fill_by_one.len() {
        println!( "{}. {}", i, fill_by_one[ i ] );
    }

    // how much memory is used by array
    println!( "Array \"fill_by_one\" took up {} bytes", mem::size_of_val( &fill_by_one ) );


    // Matrix
    let mtx:[ [ f32; 3 ]; 3 ] =
    [
        [ 1.0, 0.0, 0.0 ],
        [ 0.0, 2.0, 0.0 ],
        [ 0.0, 0.0, 3.0 ]
    ];
    println!( "My Matrix: {:?}", mtx );

    // Print diagonal
    for i in 0..mtx.len() {
        for j in 0..mtx[ i ].len() {
            if i == j {
                println!("Matrix[{}][{}] = {}", i, j, mtx[ i ][ j ]);
            }
        }
    }
}