use std::f32::consts::PI;

fn main() {
    let cube = Cuboid::new(3., 3. ,3.);
    let sphere = Sphere::new(3.);

    println!("{}", cube.volume());
    println!("{}", sphere.volume());
}

pub trait Volume {
    fn volume(&self) -> f32;
}

struct Cuboid {
    height: f32,
    width: f32,
    length: f32,
}

impl Cuboid {
    pub fn new(height: f32, width: f32, length: f32) -> Cuboid {
        Cuboid {
            height: height,
            width: width,
            length: length,
        }
    }
}

impl Volume for Cuboid {
    fn volume(&self) -> f32 {
        self.height * self.length * self.width
    }
}

struct Sphere {
    radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Sphere {
        Sphere {
            radius: radius,
        }
    }
}

impl Volume for Sphere {
    fn volume(&self) -> f32 {
        4. / 3. * PI * self.radius.powf(3.)
    }
}