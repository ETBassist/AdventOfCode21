use std::fs::File;
use std::io::{Read, BufReader};

fn main() {
    if let Ok(f) = File::open("input.txt") {
        let mut buf_reader = BufReader::new(f);
        let mut contents = String::new();
        let mut binary: [Pair; 12] = [
            Pair { zero: 0, one: 0 },
            Pair { zero: 0, one: 0 },
            Pair { zero: 0, one: 0 },
            Pair { zero: 0, one: 0 },
            Pair { zero: 0, one: 0 },
            Pair { zero: 0, one: 0 },
            Pair { zero: 0, one: 0 },
            Pair { zero: 0, one: 0 },
            Pair { zero: 0, one: 0 },
            Pair { zero: 0, one: 0 },
            Pair { zero: 0, one: 0 },
            Pair { zero: 0, one: 0 },
        ];

        let _result = buf_reader.read_to_string(&mut contents); 

        let input = contents.trim().split('\n');

        for line in input {
            for (index, num) in line.chars().enumerate() {
                match num {
                    '0' => binary[index].zero += 1,
                    '1' => binary[index].one += 1,
                    _ => println!("Unable to match against: {}", num),
                }
            }
        }

        let mut gamma: String = String::new();
        let mut epsilon: String = String::new();

        for pair in binary.iter() {
            if pair.zero > pair.one {
                gamma.push_str("0");
                epsilon.push_str("1");
            } else {
                gamma.push_str("1");
                epsilon.push_str("0");
            }
        }

        println!("gamma binary: {}", gamma);
        println!("epsilon binary: {}", epsilon);
        let decimal_gamma = isize::from_str_radix(&gamma, 2).unwrap();
        let decimal_epsilon = isize::from_str_radix(&epsilon, 2).unwrap();
        println!("Gamma: {}", decimal_gamma);
        println!("Epsilon: {}", decimal_epsilon);
        println!("Total: {}", decimal_epsilon * decimal_gamma);
    }
}

#[derive(Debug)]
struct Pair {
    zero: u16,
    one: u16
}
