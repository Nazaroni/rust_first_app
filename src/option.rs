pub fn option() {
    // Option<T> (T - type)
    let x = 3.0;
    let y = 2.0;
    // let y = 0.0;

    // Some(z) None
    let result: Option<f64> = if y != 0.0 { Some( x / y) } else { None };
    println!( "{:?}", result );

    match result {
        Some( z ) => println!( "{} / {} = {}", x, y, z ),
        None => println!( "cannot divide {} by {}", x, y )
    }

    // if let / while let is true in line 8
    if let Some( karabaka ) = result { println!( "karabaka = {}", karabaka ); }
}