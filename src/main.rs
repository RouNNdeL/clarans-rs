use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;
use std::ops;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn get_neighbor<'a>(points: &'a Vec<Point>, medoids: &Vec<&'a Point>) -> Vec<&'a Point> {
    let mut rng = rand::thread_rng();
    let med_idx = rng.gen_range(0..medoids.len());
    let mut cloned = medoids.clone();

    cloned[med_idx] = points
        .choose(&mut rand::thread_rng())
        .expect("Unable to select a random point");
    cloned
}

fn closest_medoid<'a>(point: &'a Point, medoids: &Vec<&'a Point>) -> (&'a Point, f64) {
    let mut min_distance = f64::INFINITY;
    let mut min_point = medoids[0];

    for m in medoids {
        let distance = point.distance(m);
        if distance < min_distance {
            min_distance = distance;
            min_point = m;
        }
    }

    (min_point, min_distance)
}

fn compute_total_cost(points: &Vec<Point>, medoids: &Vec<&Point>) -> f64 {
    points.iter().map(|p| closest_medoid(&p, medoids).1).sum()
}

fn init_medoids(points: &Vec<Point>, count: usize) -> Vec<&Point> {
    points
        .choose_multiple(&mut rand::thread_rng(), count)
        .collect()
}

fn clarans(
    points: &Vec<Point>,
    num_clusters: usize,
    num_local: usize,
    max_neighbors: usize,
) -> Vec<&Point> {
    let mut medoids: Vec<&Point> = Vec::new();
    let mut cost = f64::INFINITY;
    for _ in 0..num_local {
        let mut current_medoids = init_medoids(&points, num_clusters);
        let mut current_cost = compute_total_cost(&points, &current_medoids);

        for _ in 0..max_neighbors {
            let neighbor = get_neighbor(&points, &current_medoids);
            let neighbor_cost = compute_total_cost(&points, &neighbor);
            if neighbor_cost < current_cost {
                current_medoids = neighbor;
                current_cost = neighbor_cost;
            }
        }

        if current_cost < cost {
            println!("Improved cost from {cost} to {current_cost}");
            medoids = current_medoids;
            cost = current_cost;
        }
    }

    medoids
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
    let medoids = clarans(&points, CLUSTER_COUNT, MINIMA_COUNT, NEIGHBOR_MAX);

    dbg!(&medoids);
}
