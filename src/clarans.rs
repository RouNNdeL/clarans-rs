use rand::seq::SliceRandom;
use rand::Rng;
use std::thread;

pub trait Distance<T = Self> {
    fn distance(self, other: T) -> f64;
}

fn get_neighbor<'a, T>(points: &'a [T], medoids: &[&'a T]) -> Vec<&'a T> {
    let mut rng = rand::thread_rng();
    let med_idx = rng.gen_range(0..medoids.len());
    let mut cloned = medoids.to_vec();

    cloned[med_idx] = points
        .choose(&mut rand::thread_rng())
        .expect("Unable to select a random point");
    cloned
}

fn closest_medoid<'a, T>(point: &'a T, medoids: &[&'a T]) -> (&'a T, f64)
where
    &'a T: Distance<&'a T>,
{
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

fn compute_total_cost<'a, T>(points: &'a [T], medoids: &[&'a T]) -> f64
where
    &'a T: Distance<&'a T>,
{
    points.iter().map(|p| closest_medoid(&p, medoids).1).sum()
}

fn init_medoids<T>(points: &[T], count: usize) -> Vec<&T> {
    points
        .choose_multiple(&mut rand::thread_rng(), count)
        .collect()
}

pub fn calculate_medoids<'a, T>(
    points: &'a [T],
    num_clusters: usize,
    minima: usize,
    max_neighbors: usize,
) -> (Vec<&'a T>, f64)
where
    &'a T: Distance<&'a T>,
{
    let mut medoids: Vec<&T> = Vec::new();
    let mut cost = f64::INFINITY;
    for _ in 0..minima {
        let mut current_medoids = init_medoids(points, num_clusters);
        let mut current_cost = compute_total_cost(&points, &current_medoids);

        for _ in 0..max_neighbors {
            let neighbor = get_neighbor(&points, &current_medoids);
            let neighbor_cost = compute_total_cost(&points, &neighbor);
            if neighbor_cost < current_cost {
                current_medoids = neighbor;
                current_cost = neighbor_cost;
                break;
            }
        }

        if current_cost < cost {
            medoids = current_medoids;
            cost = current_cost;
        }
    }

    (medoids, cost)
}

pub fn calculate_medoids_fast<'a, T>(
    points: &'a [T],
    num_clusters: usize,
    minima: usize,
    max_neighbors: usize,
    num_threads: usize,
) -> (Vec<&'a T>, f64)
where
    &'a T: Distance<&'a T> + Sync,
    T: Sync,
{
    let mut overall_best_medoids = Vec::new();
    let mut overall_best_cost = f64::INFINITY;

    thread::scope(|s| {
        let mut handles = vec![];

        for thread_id in 0..num_threads {
            let handle = s.spawn(move || {
                let remainder = minima % num_threads;
                let local_minima = minima / num_threads + if thread_id < remainder { 1 } else { 0 };
                calculate_medoids(&points, num_clusters, local_minima, max_neighbors)
            });
            handles.push(handle);
        }

        for handle in handles {
            let (local_medoids, local_cost) = handle.join().unwrap();
            if local_cost < overall_best_cost {
                overall_best_medoids = local_medoids;
                overall_best_cost = local_cost;
            }
        }
    });

    (overall_best_medoids, overall_best_cost)
}
