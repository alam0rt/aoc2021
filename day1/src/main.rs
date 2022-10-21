use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    let mut i = 0;
    let measurements = input("./src/input");
    for elem in measurements.windows(2) {
	if elem[0] < elem[1] { i = i + 1; };
	println!("{} {}", elem[0], elem[1]);
    }
    println!("result: {}", i);
}

fn input(path: &str) -> Vec<i64> {
    let mut contents = String::new();
    let f = File::open(path).expect("failed to open path");
    let mut buf_reader = BufReader::new(f);

    buf_reader
	.read_to_string(&mut contents)
	.expect("unable to read file into buffer");

    let list = contents.lines()
	.map(|s| {s.parse::<i64>().unwrap()} )
	.collect::<Vec<i64>>();

    return list
}
