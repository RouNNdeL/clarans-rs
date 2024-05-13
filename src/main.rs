mod clarans;

use rand::Rng;
use std::fmt;
use std::ops;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl<'a, 'b> clarans::Distance<&'b Point> for &'a Point {
    fn distance(self, other: &'b Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn create_random_points(count: usize, value_range: ops::Range<f64>) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut points: Vec<Point> = Vec::with_capacity(count);

    for _ in 0..count {
        let point = Point {
            x: rng.gen_range(value_range.clone()),
            y: rng.gen_range(value_range.clone()),
        };
        points.push(point);
    }

    points
}

fn main() {
    const DATA_SIZE: usize = 3000;
    const CLUSTER_COUNT: usize = 1;
    const MINIMA_COUNT: usize = 15;
    const NEIGHBOR_MAX: usize = 10;

    let points = create_random_points(DATA_SIZE, -10.0..10.0);
    let medoids = clarans::clarans(&points, CLUSTER_COUNT, MINIMA_COUNT, NEIGHBOR_MAX);

    dbg!(&medoids);
}
