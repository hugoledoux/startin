use super::Point3d;
use std::ops::{Add, Sub};

pub fn orient2d(a: &Point3d, b: &Point3d, c: &Point3d) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- colinear = 0
    let re: f64 = ((a.x - c.x) * (b.y - c.y)) - ((a.y - c.y) * (b.x - c.x));
    if re.abs() < 1e-12 {
        return 0;
    } else if re > 0.0 {
        return 1;
    } else {
        return -1;
    }
}

pub fn incircle(a: &Point3d, b: &Point3d, c: &Point3d, p: &Point3d) -> i8 {
    //-- p is INSIDE   == +1
    //-- p is OUTSIDE  == -1
    //-- p is ONCIRCLE == 0
    let at = (
        a.x - p.x,
        a.y - p.y,
        (a.x * a.x + a.y * a.y) - (p.x * p.x + p.y * p.y),
    );
    let bt = (
        b.x - p.x,
        b.y - p.y,
        (b.x * b.x + b.y * b.y) - (p.x * p.x + p.y * p.y),
    );
    let ct = (
        c.x - p.x,
        c.y - p.y,
        (c.x * c.x + c.y * c.y) - (p.x * p.x + p.y * p.y),
    );
    let i = at.0 * (bt.1 * ct.2 - bt.2 * ct.1);
    let j = at.1 * (bt.0 * ct.2 - bt.2 * ct.0);
    let k = at.2 * (bt.0 * ct.1 - bt.1 * ct.0);
    let re = i - j + k;
    // println!("INCIRCLE TEST: {}", re);
    if re.abs() < 1e-12 {
        return 0;
    } else if re > 0.0 {
        return 1;
    } else {
        return -1;
    }
}

pub struct Vector {
    pub x: f64,
    pub y: f64,
}
impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Vector {
    pub fn new(ax: f64, ay: f64, bx: f64, by: f64) -> Vector {
        Vector {
            x: bx - ax,
            y: by - ay,
        }
    }
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn normalise(&mut self) {
        let l = self.length();
        if l == 0.0 {
            panic!("Vector of length=0 can't be normalised.");
        }
        self.x = self.x / l;
        self.y = self.y / l;
    }
}

pub fn vector_projection(a: &Point3d, b: &Point3d, c: &Point3d) -> f32 {
    let mut p = Vector::new(a.x, a.y, b.x, b.y);
    let mut q = Vector::new(a.x, a.y, c.x, c.y);

    p.normalise();

    0.22
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalise() {
        let mut v = Vector::new(0.0, 0.0, 1.0, 0.0);
        assert_eq!(v.length(), 1.0);
        v = Vector::new(1.0, 1.0, 1.0, 5.0);
        assert_eq!(v.length(), 4.0);
    }
    #[test]
    #[should_panic]
    fn normalise_panic() {
        let mut v = Vector::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(v.length(), 1.0);
    }
}
