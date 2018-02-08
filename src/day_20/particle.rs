type Coords = (i32, i32, i32);

#[derive(Debug)]
pub struct Particle {
    pub position: Coords,
    pub velocity: Coords,
    acceleration: Coords,
}

impl Particle {
    pub fn new(position: Coords, velocity: Coords, acceleration: Coords) -> Particle {
        Particle {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }
}
