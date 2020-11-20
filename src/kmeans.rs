pub mod point;

use point::Point;
use std::collections::HashSet;
use rand::distributions::{Distribution, Uniform};

fn get_k_random_points(points: &Vec<Point>, k: usize) -> Vec<Point> {
    let mut selecteds: Vec<Point> = Vec::new();

    let mut rng = rand::thread_rng();
    let dist = Uniform::from(1..points.len());
    let mut set = HashSet::new();

    while selecteds.len() < k as usize {
        let s = dist.sample(&mut rng);
        if !set.contains(&s) {
            set.insert(s);
            selecteds.push(points[s]);
        }
    }

    return selecteds;
}


fn get_k_close_centroid(point: &Point, centroids: &Vec<Point>) -> usize {
    let mut min :usize = 0;
    let mut min_dist = point.distance(&centroids[min]);

    for (k, centroid) in centroids.iter().enumerate() {
        let dist = point.distance(centroid);
        if dist < min_dist {
            min = k;
            min_dist = dist;
        }
    }
    min
}

fn converged(current_centroids: &Vec<Point>, new_centroids: &Vec<Point>) -> bool {
    for new_centroid in new_centroids.iter() {
        if ! current_centroids.contains(&new_centroid) {
            return false;
        }
    }
    return true;
}

fn assign_clusters(centroids: &Vec<Point>, points: &Vec<Point>, assigned_cluster: &mut Vec<usize>) {
    for (i, point) in points.iter().enumerate() {
        let close_centroid = get_k_close_centroid(point, &centroids);
        assigned_cluster[i] = close_centroid;
    }
}

fn compute_new_centroids(k: usize, points: &Vec<Point>, assigned_cluster: &Vec<usize>) -> Vec<Point> {
    let mut new_centroids = vec![Point::origin(); k];
    let mut points_in_cluster = vec![0; k];

    for (i, point) in points.iter().enumerate() {
        let cluster = assigned_cluster[i];
        new_centroids[cluster] += point;
        points_in_cluster[cluster] += 1;
    }

    for (i, mut new_centroid) in new_centroids.iter_mut().enumerate() {
        new_centroid /=  points_in_cluster[i];
    }

    return new_centroids;
}

pub fn kmeans(points: Vec<Point>, k: usize) -> (u32, Vec<Point>) {

    let mut current_centroids: Vec<Point> = get_k_random_points(&points, k);
    let mut assigned_cluster: Vec<usize> = vec![0; points.len()];

    println!("Start centroids: {:?}", current_centroids);

    let mut iterations = 0;

    loop {
        assign_clusters(&current_centroids, &points, &mut assigned_cluster);

        let new_centroids = compute_new_centroids(k, &points, &assigned_cluster);

        if converged(&current_centroids, &new_centroids) {
            break;
        }

        current_centroids = new_centroids;
        iterations += 1;
    }

    return (iterations, current_centroids);
}


#[cfg(test)]
mod test {
    use super::Point;
    use crate::kmeans::{converged, get_k_close_centroid, compute_new_centroids};

    #[test]
    fn test_get_k_close_centroid() {
        // fn get_k_close_centroid(point: &Point, centroids: &Vec<Point>) -> usize {
        let points = vec![Point::new(0.0, 0.0), Point::new(2.0, 2.0), Point::new(-2.0, -2.0)];

        assert_eq!(get_k_close_centroid(&Point::new(0.1, 0.1), &points), 0);
        assert_eq!(get_k_close_centroid(&Point::new(1.0, 1.0), &points), 0);
        assert_eq!(get_k_close_centroid(&Point::new(1.1, 1.1), &points), 1);
        assert_eq!(get_k_close_centroid(&Point::new(-1.1, -1.1), &points), 2);

    }

    #[test]
    fn test_compute_new_centroids() {
        // fn compute_new_centroids(k: usize, points: &Vec<Point>, assigned_cluster: &Vec<usize>) -> Vec<Point> {
        let points = vec![Point::new(7.0, 9.0), Point::new(9.0, 2.0), Point::new(-2.0, -6.3), Point::new(4.0, 6.0)];
        let centroids = compute_new_centroids(2, &points, &vec![0,0,1,1]);
        assert_eq!(centroids, vec![Point::new(8.0, 5.5), Point::new(1.0, -0.1499999999999999)])
    }

    #[test]
    fn test_converged() {
        // converged(current_centroids: &Vec<Point>, new_centroids: &Vec<Point>) -> bool
        let points_a = vec![Point::new(7.0, 9.0), Point::new(9.0, 2.0), Point::new(-2.0, -6.3), Point::new(4.0, 6.0)];
        let points_b = vec![Point::new(7.0, 9.0), Point::new(9.0, 2.0), Point::new(-2.0, -6.3), Point::new(4.0, 6.0)];
        let points_c = vec![Point::new(7.0, 9.0), Point::new(9.0, -2.1), Point::new(-2.0, -6.3), Point::new(4.0, 6.0)];

        assert_eq!(converged(&points_a, &points_b), true);
        assert_ne!(converged(&points_a, &points_c), true);
    }
}