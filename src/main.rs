struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point{ x: self.x, y: self.y }
    }
}


struct Wire {
    currentPoint: Point,
}

impl Wire {
    fn travel(&self, x: i32, y: i32) {

    }
}

fn new_wire (origin: Point) -> Wire {
    Wire{ currentPoint: origin.clone() }
}

fn travel(wire: &mut Wire, directions: Vec<String>) {

}



fn main () {
    let input1: Vec<String> = vec![String::from("R75"),String::from("D30"),String::from("R83"),String::from("U83"),String::from("L12"),String::from("D49"),String::from("R71"),String::from("U7"),String::from("L72")];
    let input2: Vec<String> = vec![String::from("U62"),String::from("R66"),String::from("U55"),String::from("R34"),String::from("D71"),String::from("R55"),String::from("D58"),String::from("R83")];
    let origin = Point { x: 0, y: 0 };

    let wire1 = new_wire(origin.clone());


    for dir in input1 {
        let parts = &dir.split_at(1);
        match parts.0 {
            "L" => {

            },
            "R" => ,
            "U" => ,
            "D" => ,
            _ => panic!("Invalid command {}", dir),
        };

        // let bytes = dir.as_bytes();
        // match bytes[0] {
        //     b'R' => println!("Right"),
        //     _ => println!("Stuff"),
        // }
    }

    // let wire2 = new_wire(origin.clone(), input2);

    println!("hi {:?}", 0);
}