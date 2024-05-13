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

#[derive(Debug)]
struct PointN<const N: usize>([f64; N]);

impl<'a, 'b, const N: usize> clarans::Distance<&'b PointN<N>> for &'a PointN<N> {
    fn distance(self, other: &'b PointN<N>) -> f64 {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

impl<const N: usize> fmt::Display for PointN<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let joined = self
            .0
            .iter()
            .map(|&num| num.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "({joined})")
    }
}

fn create_random_points<const N: usize>(count: usize, value_range: ops::Range<f64>) -> Vec<PointN<N>> {
    let mut rng = rand::thread_rng();
    let mut points: Vec<PointN<N>> = Vec::with_capacity(count);

    for _ in 0..count {
        let coords = {
            let mut temp = [0.0; N];
            for i in 0..N {
                temp[i] = rng.gen_range(value_range.clone());
            }
            temp
        };
        points.push(PointN::<N>(coords));
    }

    points
}

fn main() {
    const DATA_SIZE: usize = 5000;
    const CLUSTER_COUNT: usize = 10;
    const MINIMA_COUNT: usize = 100;
    const NEIGHBOR_MAX: usize = 100;

    let points = create_random_points::<10>(DATA_SIZE, -10.0..10.0);
    let medoids = clarans::clarans(&points, CLUSTER_COUNT, MINIMA_COUNT, NEIGHBOR_MAX);

    dbg!(&medoids);
}
