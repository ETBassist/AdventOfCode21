use std::fs::File;
use std::io::{Read, BufReader};

fn main() {
    if let Ok(f) = File::open("input.txt") {
        let mut buf_reader = BufReader::new(f);
        let mut contents = String::new();

        let _result = buf_reader.read_to_string(&mut contents); 

        let mut x_coordinate: i64 = 0;
        let mut y_coordinate: i64 = 0;
        let mut aim: i64 = 0;
        let mut depth: i64 = 0;

        let input = contents.trim().split("\n");

        for line in input {
            let chars: Vec<&str> = line.trim().split(" ").collect();
            let first_letter: &str = chars[0];
            let num: i64 = chars[chars.len() - 1].parse::<i64>().unwrap();
            match first_letter {
                "forward" => {
                    x_coordinate += num;
                    depth += aim * num;
                },
                "up" => {
                    y_coordinate -= num;
                    aim -= num;
                },
                "down" => {
                    y_coordinate += num;
                    aim += num;
                },
                _ => println!("Something went wrong"),
            }
        }

        println!("The final position is: {}", x_coordinate * y_coordinate);
        println!("The final position with aiming is: {}", x_coordinate * depth);
    }
}
