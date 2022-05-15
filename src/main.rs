#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

mod stack_and_heap;
mod control_flow;
mod while_and_loop;
mod for_loop;
mod match_statement;
mod structures;
mod enumerations;
mod option;
mod arrays;
mod vectors;
mod slices;
mod strings;
mod tuples;
mod formatted_print;
mod scope_and_shadowing;
mod debug;
mod tmp;


// This is a comment in the Rust :D

/* This is the same comment
 * like in TypeScript
 * This is so beautiful 😁
 */



fn operators () {

    // arithmetic
    let mut a = 2 + 3 * 4;
    println!( "{}", a );
    a += 2;
    a -= 1;

    println!( "remainder of {} / {} = {}", a, 3, ( a % 3 ) );

    let a_cubed = i32::pow( a, 3 );
    println!( "{} cubed is {}", a, a_cubed );

    let b = 2.5;
    let b_cubed = f64::powi( b, 3 );
    let b_to_pi = f64::powf( b, std::f64::consts::PI );
    println!( "{} cubed = {},  {}^pi = {}", b, b_cubed, b, b_to_pi );


    // bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
    println!( "1|2 = {}", c );

    let two_to_10 = 1 << 10;
    println!( "2^10 = {}", two_to_10);


    // logical ( < > <= >= ==)
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    println!( "pi_less_4 is: {}", pi_less_4 );

    let x = 5;
    let x_is_5 = x == 5;
    println!( "x_is_5 is: {}", x_is_5 );

}


const MEANING_OF_LIFE:u8 = 42; // no fixed address (!!!preferable!!! way to use constants!!!)

// this constant has the memory address. Can be mutable in "unsafe" mode
static mut ANOTHER_CONSTANT:i32 = 4321;
fn constants () {
    println!( "{}", MEANING_OF_LIFE );
    unsafe {
        println!( "{}", ANOTHER_CONSTANT );
        ANOTHER_CONSTANT = 6790;
        println!( "{}", ANOTHER_CONSTANT );
    }
}


fn main() {
    tmp::tmp();
    // stack_and_heap::stack_and_heap();
    // control_flow::if_statement();
    // while_and_loop::while_and_loop();
    // for_loop::for_loop();
    // match_statement::match_statement();
    // structures::structures();
    // enumerations::enumerations();
    // option::option();
    // arrays::arrays();
    // vectors::vectors();
    // slices::slices();
    // strings::strings();
    // tuples::tuples();
    // formatted_print::formatted_print();
    // debug::debug();
}

