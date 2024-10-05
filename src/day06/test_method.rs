pub struct Circle {
    radius: f64,
}

impl Circle {

    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

pub enum Direction {
    EAST,
    SOUTH,
    WEST,
    NORTH,
}

impl Direction {
    pub fn get_name(&self) -> &str {
        match self {
            Direction::EAST => "east",
            Direction::SOUTH => "south",
            Direction::WEST => "west",
            Direction::NORTH => "north",
        }
    }
}

fn test() {
    println!("Hello, world!");
    let circle = Circle::new(5f64);

    let area = circle.area();
    println!("{}", area);

    let direction = Direction::EAST;
    let name = direction.get_name();
    println!("{}", name);

    let direction = Direction::SOUTH;
    let name = direction.get_name();
    println!("{}", name);

}