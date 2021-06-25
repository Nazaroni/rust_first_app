pub fn structures() {
    struct Point {
        x: f64,
        y: f64
    }

    // nested structures
    struct Line {
        start: Point,
        end: Point
    }

    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let my_line = Line { start: p, end: p2 };

    // println!( "Point p is at ({}, {})", p.x, p.y );
    println!( "Line start (x:{}, y:{}) / end (x:{}, y:{}) coordinates",
              my_line.start.x,
              my_line.start.y,
              my_line.end.x,
              my_line.end.y );

}