pub mod kmeans;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::kmeans::point::Point;
use crate::kmeans::kmeans;
use std::time::{SystemTime};


fn main() {
    let lines = read_lines("iris.data").expect("Error reading lines.");

    let mut points : Vec<Point> = Vec::new();


    for line_result in lines {
        match line_result {
            Ok(line) => {
                let parts : Vec<&str> = line.split(",").collect();
                let x = parts[0].parse::<f64>().expect("Incorrect float for x coordinate");
                let y = parts[1].parse::<f64>().expect("Incorrect float for x coordinate");
                let point = Point::new(x, y);

                points.push(point);
            }
            Err(error) => {
                println!("Error reading from file: {}", error);
            }
        }
    }

    let before = SystemTime::now();
    let (iterations, current_centroids) = kmeans(points, 6);
    let elapsed_time = before.elapsed().expect("Error measuring time");

    println!("\nFinal Centroids =: {:?}", current_centroids);
    println!("\nFinished in {} iterations, that required {}s", iterations, elapsed_time.as_secs_f64());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
