mod clarans;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops;

use std::io::{self, Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "clarans")]
struct Opt {
    /// Input CSV file containing data points, use '-' for stdin
    #[structopt(short = "i", parse(from_os_str), default_value = "-")]
    input: PathBuf,

    /// Output CSV file to store the resulting medoids, use '-' for stdout
    #[structopt(short = "o", parse(from_os_str), default_value = "-")]
    output: PathBuf,

    /// Number of clusters
    #[structopt(short = "c", long, default_value = "4")]
    num_clusters: usize,

    /// Minima parameter
    #[structopt(short = "m", long, default_value = "1000")]
    minima: usize,

    /// Maximum neighbors parameter
    #[structopt(short = "n", long, default_value = "100")]
    max_neighbors: usize,

    /// Number of threads
    #[structopt(short = "t", long, default_value = "8")]
    num_threads: usize,
}

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
    let opt = Opt::from_args();
    run_clarans(
        &opt.input,
        &opt.output,
        opt.num_clusters,
        opt.minima,
        opt.max_neighbors,
        opt.num_threads,
    );
}

fn run_clarans(
    input_data: &PathBuf,
    output_medoids: &PathBuf,
    num_clusters: usize,
    minima: usize,
    max_neighbors: usize,
    num_threads: usize,
) {
    let mut points = Vec::new();
    if input_data.to_str() == Some("-") {
        let stdin = io::stdin();
        let mut reader = csv::Reader::from_reader(stdin.lock());
        for p in reader.deserialize() {
            let point: Point = p.expect("Unable to parse CSV");
            points.push(point);
        }
    } else {
        let mut reader = csv::Reader::from_path(input_data).expect("Unable to open file");
        for p in reader.deserialize() {
            let point: Point = p.expect("Unable to parse CSV");
            points.push(point);
        }
    }

    if points.len() == 0 {
        panic!("No points provided");
    }


    let (result_points, best_cost) =
        clarans::calculate_medoids_fast(&points, num_clusters, minima, max_neighbors, num_threads);

    println!("Best cost: {best_cost}");
    if output_medoids.to_str() == Some("-") {
        let stdout = io::stdout();
        let mut writer = csv::Writer::from_writer(stdout.lock());
        for p in result_points {
            writer
                .serialize(p)
                .expect("Unable to serialize resulting point");
        }
        writer.flush().expect("Unable to flush");
    } else {
        let mut writer = csv::Writer::from_path(output_medoids).expect("Unable to open file");
        for p in result_points {
            writer
                .serialize(p)
                .expect("Unable to serialize resulting point");
        }
        writer.flush().expect("Unable to flush");
    }
}
