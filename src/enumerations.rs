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
}