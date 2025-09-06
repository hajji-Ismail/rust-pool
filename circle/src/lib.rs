#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius: r,
        }
    }
    pub fn area(&self) -> f64 {
        let pi = std::f64::consts::PI;
        self.radius * self.radius * pi
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn intersect(&self, circle1: Circle) -> bool {
        if (self.radius - circle1.radius).abs()
            <= ((self.center.0 - circle1.center.0).powf(2.0)
                + (self.center.1 - circle1.center.1).powf(2.0))
            .sqrt()
            && ((self.center.0 - circle1.center.0).powf(2.0)
                + (self.center.1 - circle1.center.1).powf(2.0))
            .sqrt()
                <= (circle1.radius + self.radius)
        {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, point_b: Point) -> f64 {
        ((point_b.0 - self.0).powf(2.0) + (point_b.1 - self.1).powf(2.0)).sqrt()
    }
}
