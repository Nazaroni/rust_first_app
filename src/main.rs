// This is a comment in the Rust :D

/* This is the same comment
 * like in TypeScript
 * This is so beautiful 😁
 */

fn main() {
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
    println!("The day is {my_day}", my_day="ffff");

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
}
