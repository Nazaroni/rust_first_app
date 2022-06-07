pub fn enumerations() {
    enum Color {
        Red,
        Green,
        Blue,
        RgbColor( u8, u8, u8 ),                                     // Tuple
        CMYKColor{ cyan:u8, magenta:u8, yellow:u8, black:u8 }       // Struct
    }

    fn enums() {
        // let my_color:Color = Color::RgbColor(0,0,0);
        let my_color:Color = Color::CMYKColor{ cyan: 0, magenta: 0, yellow: 0, black:255 };

        match my_color {
            Color::Red      =>  println!( "r" ),
            Color::Green    =>  println!( "g" ),
            Color::Blue     =>  println!( "b" ),
            Color::RgbColor( 0, 0, 0 )
            | Color::CMYKColor{ cyan: _, magenta: _, yellow: _, black: 255 } => println!( "black" ),
            Color::RgbColor( r, g, b ) => println!( "rgb({}, {}, {})", r, g, b ),
            _ => () // do nothing
        }
    }

    enums();

    // https://docs.microsoft.com/en-us/learn/modules/rust-create-program/5-enum-variants

    // Define a tuple struct
    #[derive(Debug)]
    struct KeyPress(String, char);

    // Define a classic struct
    #[derive(Debug)]
    struct MouseClick { x: i64, y: i64 }

    // Define the WebEvent enum variants to use the data from the structs
    // and a boolean type for the page Load variant
    #[derive(Debug)]
    enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);

    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);

    // Instantiate WebEvent enum variants
    // Set the boolean page Load value to true
    let we_load = WebEvent::WELoad(true);
    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);

    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);
}