// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::ops::Add;

#[derive(Debug)]
pub struct Point {
    x: u32,
    y: u32
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point{x, y}
    }

    fn dist(&self, p2: &Point) -> f64 {
        f64::sqrt(
            (
                (p2.x - self.x).pow(2) +
                (p2.y - self.y).pow(2)
            ) as f64)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

}

#[derive(Debug)]
pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    fn new() -> Polygon {
        Polygon { points: vec![]  }
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p)
    }

    fn left_most_point(&self) -> Option<&Point> {
        let mut smallest_x =  None;
        for p in self.points.iter() {
            if smallest_x.is_none() {
                smallest_x = Option::from(p)
            } else if smallest_x.unwrap().x > p.x {
                smallest_x = Option::from(p)
            }
        }
        smallest_x
    }
}

#[derive(Debug)]
pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    // add methods
    fn new(center: Point, radius: f64) -> Circle {
        Circle{center, radius};
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    // #[test]
    // fn test_shape_circumferences() {
    //     let mut poly = Polygon::new();
    //     poly.add_point(Point::new(12, 13));
    //     poly.add_point(Point::new(16, 16));
    //     let shapes = vec![
    //         Shape::from(poly),
    //         Shape::from(Circle::new(Point::new(10, 20), 5)),
    //     ];
    //     let circumferences = shapes
    //         .iter()
    //         .map(Shape::circumference)
    //         .map(round_two_digits)
    //         .collect::<Vec<_>>();
    //     assert_eq!(circumferences, vec![10.0, 31.42]);
    // }
}

#[allow(dead_code)]
fn main() {}
