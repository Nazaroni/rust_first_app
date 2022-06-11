fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        // Return early
        return 0;
    }
    num / 5
}

fn goodbye(message: &str) {
    println!("\n{}", message);
}

pub fn functions() {
    // https://docs.microsoft.com/en-us/learn/modules/rust-create-program/6-functions
    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));

    let formal = "Formal: Good bye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);
}