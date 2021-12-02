use std::fs::File;
use std::io::{Read, BufReader};

fn main() {
    if let Ok(f) = File::open("input.txt") {
        let mut buf_reader = BufReader::new(f);
        let mut contents = String::new();

        let _result = buf_reader.read_to_string(&mut contents); 

        let mut count: u16 = 0;
        let input = contents.trim().split("\n");
        let mut previous_num: u16 = 0;

        for line in input {
            let num = line.parse::<u16>().unwrap();

            if previous_num == 0 {
                previous_num = num;
            }

            if num > previous_num {
                count += 1;
            }

            previous_num = num;
        }

        println!("The final tally is: {}", count);
    }
}
