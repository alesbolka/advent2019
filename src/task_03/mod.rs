mod point;
mod plane;
mod wire;
pub use point::Point;
pub use plane::Plane;
pub use wire::Wire;


pub fn ex1 () {

}

pub fn demo () {
    let x = Plane::X;
    if let Plane::X = x {
        println!("Foo");
    }
    // let input1: Vec<String> = vec![String::from("R8"),String::from("U5"),String::from("L5"),String::from("D3")];
    // let input2: Vec<String> = vec![String::from("U7"),String::from("R6"),String::from("D4"),String::from("L4")];
    // let inp = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(",");
    let input1: Vec<String> = vec![String::from("R75"),String::from("D30"),String::from("R83"),String::from("U83"),String::from("L12"),String::from("D49"),String::from("R71"),String::from("U7"),String::from("L72")];
    let input2: Vec<String> = vec![String::from("U62"),String::from("R66"),String::from("U55"),String::from("R34"),String::from("D71"),String::from("R55"),String::from("D58"),String::from("R83")];
    let origin = Point::new(0, 0);

    let mut wire1 = Wire::new(origin.clone());
    wire1.path(input1);
    let mut wire2 = Wire::new(origin.clone());
    wire2.path(input2);

    let points = Wire::find_intersects(&wire1, &wire2);
    let x: i32 = match points.iter().map(|a| a.dist(&origin)).min() {
        Some(x) => x,
        None => -1,
    };
    println!("{:?}", x);
}