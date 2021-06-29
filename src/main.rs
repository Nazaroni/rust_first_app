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

use std::fmt; // import 'fmt'

// This is a comment in the Rust :D

/* This is the same comment
 * like in TypeScript
 * This is so beautiful 😁
 */

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

#[derive(Debug)]
struct MinMax(i64, i64);

struct List(Vec<i32>);

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, and a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }
        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn formatted_print() {
    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    println!("Epson Forever! The best printer to print 'println!'");
    println!("Ah! Forgot to mention - I'm a Rustacean!");

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    let my_day = 21;
    println!("The day is {}", my_day);

    // there is own scope for "my_day"... it's not the same "my_day" i32 with 21.
    println!("The day is {my_day}", my_day="flower");

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    println!("{} of {:b} people know binary, the other half doesn't", 1, 15);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}Oo{number2:>width2$}", number=1, width=2, number2=3, width2=3);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=40);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    print!("Now {:?} will print. ", Structure(13));
    print!("Now {:?} will print", Structure(3));
    println!("{:?} month in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor\'s");
    println!("Now {:?} will print", Deep(Structure(7)));

    let name= "Peter";
    let age= 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
    // Normal print
    println!("{:?}", peter);

    let minimax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minimax);
    println!("Debug: {:?}", minimax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };
    let complex = Complex { real: 3.3, imag: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    println!("Compare Complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}


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

fn scope_and_shadowing() {
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
    strings::strings();
}