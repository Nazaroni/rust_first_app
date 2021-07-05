// more information: https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html

#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable also.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

pub fn debug() {
    println!( "{:?} month in a year.", 12 );
    println!( "{1} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's" );

    // Next line cause an error: required by `std::fmt::Display::fmt`
    // println!( "Now {} will print", Structure(9) );

    // `Structure` is printable!
    println!( "Now {:?} will print", Structure(9) );
    println!( "Now {:?} will print!", Deep( Structure( 7 ) ) );

    let name= "Peter";
    let age= 27;
    let peter = Person { name, age };

    // "pretty printing" with {:#?}.
    println!("{:#?}", peter);
    // Normal print
    println!("{:?}", peter);
}