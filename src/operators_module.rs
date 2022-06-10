pub fn operators_module() {
    // arithmetic
    let mut a = 2 + 3 * 4;  // 14
    println!( "{}", a );    // 14
    a += 2; // 16
    a -= 1; // 15

    println!( "remainder of {} / {} = {}", a, 3, ( a % 3 ) );   // remainder of 15 / 3 = 0

    let a_cubed = i32::pow( a, 3 ); // 3375
    println!( "{} cubed is {}", a, a_cubed );   // 15 cubed is 3375

    let b = 2.5;
    let b_cubed = f64::powi( b, 3 );    // 15.625
    let b_to_pi = f64::powf( b, std::f64::consts::PI ); // 17.78956824426124
    println!( "{} cubed = {},  {}^pi = {}", b, b_cubed, b, b_to_pi );   // 2.5 cubed = 15.625,  2.5^pi = 17.78956824426124


    // bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
    println!( "1|2 = {}", c );  // 1|2 = 3

    let two_to_10 = 1 << 10;    // 1024
    println!( "2^10 = {}", two_to_10);  // 2^10 = 1024


    // logical ( < > <= >= ==)
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    println!( "pi_less_4 is: {}", pi_less_4 );  // pi_less_4 is: true

    let x = 5;
    let x_is_5 = x == 5;    // true
    println!( "x_is_5 is: {}", x_is_5 );    // x_is_5 is: true
}