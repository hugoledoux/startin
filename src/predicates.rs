use super::Point3d;

pub fn orient2d(a: &Point3d, b: &Point3d, c: &Point3d) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- linear = 0
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
    //-- INSIDE   == +1
    //-- OUTSIDE  == -1
    //-- ONCIRCLE == 0
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
