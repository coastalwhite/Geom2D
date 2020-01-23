use std::ops;

/// A 2D Point
#[derive(Debug)]
pub struct Point(i32, i32);

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point(x, y)
    }
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y()
    }
}
impl Eq for Point {}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Point(self.x() + _rhs.x(), self.y() + _rhs.y())
    }
}

impl ops::Sub for Point {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Point(self.x() - _rhs.x(), self.y() - _rhs.y())
    }
}

impl ops::Mul<i32> for Point {
    type Output = Self;
    fn mul(self, _rhs: i32) -> Self {
        Point(self.x() * _rhs, self.y() * _rhs)
    }
}

impl ops::Mul<Point> for i32 {
    type Output = Point;
    fn mul(self, _rhs: Point) -> Point {
        Point(self * _rhs.x(), self * _rhs.y())
    }
}

impl Copy for Point { }

impl Clone for Point {
    fn clone(&self) -> Self {
        Point::new(self.x(), self.y())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_test_add() {
        let point1 = Point::new(2, 2);
        let point2 = Point::new(3,1);

        assert_eq!(point1 + point2, Point::new(5,3));
    }

    #[test]
    fn point_test_sub() {
        let point1 = Point::new(2, 2);
        let point2 = Point::new(3,1);

        assert_eq!(point1 - point2, Point::new(-1,1));
    }

    #[test]
    fn point_test_mul() {
        let point1 = Point::new(2, 2);
        let scalar = 5;

        assert_eq!(scalar*point1, Point::new(10,10));
    }
}