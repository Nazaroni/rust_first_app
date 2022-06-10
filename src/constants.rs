const MEANING_OF_LIFE:u8 = 42; // no fixed address (!!!preferable!!! way to use constants!!!)

// this constant has the memory address. Can be mutable in "unsafe" mode
static mut ANOTHER_CONSTANT:i32 = 4321;

pub fn constants() {
    println!( "{}", MEANING_OF_LIFE );          // 42
    unsafe {
        println!( "{}", ANOTHER_CONSTANT );     // 4321
        ANOTHER_CONSTANT = 6790;
        println!( "{}", ANOTHER_CONSTANT );     // 6790
    }
}