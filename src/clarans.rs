use rand::seq::SliceRandom;
use rand::Rng;

pub trait Distance {
    fn distance(&self, other: &Self) -> f64;
}

fn get_neighbor<'a, T>(points: &'a [T], medoids: &[&'a T]) -> Vec<&'a T>
where
    T: Distance,
{
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
    T: Distance,
    &'a T: Distance,
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

fn compute_total_cost<'a, T: Distance>(points: &'a [T], medoids: &[&'a T]) -> f64
where
    &'a T: Distance,
{
    points.iter().map(|p| closest_medoid(&p, medoids).1).sum()
}

fn init_medoids<T: Distance>(points: &[T], count: usize) -> Vec<&T> {
    points
        .choose_multiple(&mut rand::thread_rng(), count)
        .collect()
}

pub fn clarans<'a, T: Distance>(
    points: &'a [T],
    num_clusters: usize,
    num_local: usize,
    max_neighbors: usize,
) -> Vec<&'a T>
where
    T: Distance,
    &'a T: Distance,
{
    let mut medoids: Vec<&T> = Vec::new();
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
