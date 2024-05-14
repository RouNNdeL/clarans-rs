mod clarans;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops;

#[derive(Debug, Deserialize, Serialize)]
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

fn create_random_points<const N: usize>(
    count: usize,
    value_range: ops::Range<f64>,
) -> Vec<PointN<N>> {
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
    const INPUT_DATA: &str = "data.csv";
    const OUTPUT_MEDOIDS: &str = "result.csv";

    let mut reader = csv::Reader::from_path(INPUT_DATA).expect("Unable to open file");
    let mut points = Vec::new();
    for p in reader.deserialize() {
        let point: Point = p.expect("Unable to parse CSV");
        points.push(point)
    }

    let result = clarans::calculate_medoids(&points, 4, 100, 100);

    let mut writer = csv::Writer::from_path(OUTPUT_MEDOIDS).expect("Unable to open file");
    for p in result {
        writer
            .serialize(p)
            .expect("Unable to serialize resulting point");
    }
    writer.flush().expect("Unable to flush");
}
