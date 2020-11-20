
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let t : f64 = (self.x - other.x).powi(2) + (self.y - other.y).powi(2);
        t.sqrt()
    }

    pub fn origin() -> Point {
        Point{x:0.0, y:0.0}
    }

    pub fn new(x: f64, y: f64) -> Point {
        Point{x, y}
    }
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl std::ops::AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::AddAssign<&Point> for &mut Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.x = rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::DivAssign<usize> for &mut Point {
    fn div_assign(&mut self, scalar :usize) {
        self.x /= scalar as f64;
        self.y /= scalar as f64;
    }
}
/*
impl Clone for Point {
    fn clone(&self) -> Self {
        Point{x: self.x, y: self.y}
    }

    fn clone_from(&mut self, source: &Self) {
        self.x = source.x;
        self.y = source.y;
    }
}
*/


#[cfg(test)]
mod test {
    use super::Point;

    #[test]
    fn test_distance() {
        assert_eq!(Point{x:0.0, y:0.0}.distance(&Point{x:0.0, y:1.0}), 1.0);
        assert_eq!(Point{x:0.0, y:0.0}.distance(&Point{x:1.0, y:0.0}), 1.0);
        assert_eq!(Point{x:0.0, y:0.0}.distance(&Point{x:-1.0, y:0.0}), 1.0);
        assert_eq!(Point{x:0.0, y:0.0}.distance(&Point{x:0.0, y:-1.0}), 1.0);
        assert_eq!(Point{x:4.0, y:3.0}.distance(&Point{x:3.0, y:-2.0}), 5.0990195135927848300282241090228);
    }
}