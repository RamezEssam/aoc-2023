use std::fs::File;
use std::io::Read;
use std::env;


fn read_input_file(filepath: &str) -> String{
    let mut input_file = match File::open(filepath) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut file_content = String::new();

    input_file.read_to_string(&mut file_content).unwrap();

    file_content
}


fn parse_file_content(file_content: &str) -> Vec<Vec<i64>> {

    let lines: Vec<&str> = file_content.split("\n").collect();

    let mut  sequences: Vec<Vec<i64>> = Vec::new();

    for line in lines {
        let sequence: Vec<i64> = line.split_whitespace().map(|val| val.parse::<i64>().unwrap()).collect();

        sequences.push(sequence);
    }

    sequences
}

fn extrapolate_value(mut seq: Vec<i64>) -> i64{

    let mut all_vals: Vec<i64> = Vec::new(); 

    all_vals.push(*seq.last().unwrap());

    while !seq.iter().all(|&x| x == 0) {
        seq = seq.windows(2).map(|chunk| chunk[1] - chunk[0]).collect::<Vec<i64>>();
        all_vals.push(*seq.last().unwrap());
    }
    
    all_vals.iter().sum()
}

fn extrapolate_history(mut seq: Vec<i64>) -> i64{

    let mut all_vals: Vec<i64> = Vec::new(); 

    all_vals.push(*seq.first().unwrap());

    while !seq.iter().all(|&x| x == 0) {
        seq = seq.windows(2).map(|chunk| chunk[1] - chunk[0]).collect::<Vec<i64>>();
        all_vals.push(*seq.first().unwrap());
    }

    all_vals.iter().rev().fold(0, |acc, x| x - acc)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: main.exe <file_path>");
        panic!("No input file is provided.");
    }

    let file_path = &args[1];

    let file_content = read_input_file(file_path);

    let sequences = parse_file_content(&file_content[..]);

    let mut total_sum: i64 = 0;
    for seq in sequences.iter() {
        total_sum += extrapolate_value(seq.clone());
    }

    println!("Part 1 Solution = {}",total_sum);

    let mut total_sum: i64 = 0;

    for seq in sequences.iter() {
        total_sum += extrapolate_history(seq.clone());
    }

    println!("Part 2 Solution = {}", total_sum);

}