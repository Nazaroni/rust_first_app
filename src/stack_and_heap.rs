#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

pub struct Point {
    x: f64,
    y: f64
}

pub fn origin() -> Point {
    Point{ x: 0.0, y: 0.0 }
}

pub fn stack_and_heap () {
    // stack: short term memory
    let x = 5; // i32
    fn inc( x:i32 ) {
        x + 1;
    }
    let increased = inc( 32 );
    // println!( "{}", increased );

    // heap: long term memory. Don't have value, keep the address in stack for value located in heap
    let y = Box::new( 10 );
    println!( "y = {}", *y);

    let z = vec![ 1, 2, 3 ];
    println!( "z = {}", z[2]);

    let p1 = origin();
    // let p2 = Box:new( origin() );
    // let p3 = *p1;

    // println!( "Value of p1 is: {}", &p1 );
    println!( "p1 takes up {} bytes", mem::size_of_val( &p1 ) );
    // println!( "p2 takes up {} bytes", mem::size_of_val( &p2 ) );
    // println!( "{}", p3.x );

}