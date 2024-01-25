use std::fs::File;
use std::env;
use std::io::Read;


#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64, 
}

impl Race {
    fn to_beat(&self) -> u64 {
        let mut counter: u64 = 0;
        for i in 1..self.time{
            if i * (self.time - i) > self.record_distance {
                counter += 1
            }
        }

        counter
    }

    fn to_beat_optimized(&self) -> u64 {
        
    }
}

fn read_input_file(filepath: &str) -> String{
    let mut input_file = match File::open(filepath) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut file_content = String::new();

    input_file.read_to_string(&mut file_content).unwrap();

    file_content
}

fn parse_file_content1(file_content: String) -> Vec<Race>{

    let lines: Vec<&str> =  file_content.split("\n").collect();

    let times: Vec<&str> = lines[0].split_whitespace().collect();

    let record_distances: Vec<&str> = lines[1].split_whitespace().collect();

    let mut races: Vec<Race> = Vec::new();

    for (time, record_distance) in times.iter().zip(record_distances.iter()) {

        let race_time =  match time.parse::<u64>(){
            Ok(val) => val,
            Err(_) => continue,
        };

        let race_record_distance = match record_distance.parse::<u64>(){
            Ok(val) => val,
            Err(_) => continue,
        };

        let race = Race {
            time: race_time,
            record_distance: race_record_distance,
        };

        races.push(race);
    }

    races

}

fn parse_file_content2(file_content: String) -> Vec<Race>{

    let lines: Vec<&str> =  file_content.split("\n").collect();

    let times: Vec<&str> = lines[0].split_whitespace().collect();

    let record_distances: Vec<&str> = lines[1].split_whitespace().collect();

    let mut races: Vec<Race> = Vec::new();

    let mut race_time = String::new();

    let mut race_record_distance = String::new();

    for (time, record_distance) in times.iter().zip(record_distances.iter()) {

        match time.parse::<u64>(){
            Ok(_) => race_time.push_str(time),
            Err(_) => continue,
        };

        match record_distance.parse::<u64>(){
            Ok(_) => race_record_distance.push_str(record_distance),
            Err(_) => continue,
        };

    }

    let race_time = match race_time.parse::<u64>(){
        Ok(val) => val,
        Err(error) => panic!("Coud not convert value to integer, {}", error),
    };

    let race_record_distance = match race_record_distance.parse::<u64>(){
        Ok(val) => val,
        Err(error) => panic!("Coud not convert value to integer, {}", error),
    };

    races.push(
        Race {
            time: race_time,
            record_distance: race_record_distance,
        }
    );

    races

    

}


fn part1(races: &Vec<Race>) -> u64{
    let mut product: u64 = 1;
    for race in races {
        product =  product * (race.to_beat());
    }
    product
}


fn main(){

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: main.exe <file_path>");
        panic!("No input file is provided.");
    }

    let file_path = &args[1];

    let file_content = read_input_file(file_path);

    //let races = parse_file_content1(file_content);

    let races2 = parse_file_content2(file_content);

    //let part1_solution = part1(&races);

    let part2_solution = part1(&races2);

    //println!("Part 1 solution: {part1_solution}");

    println!("Part 2 solution: {part2_solution}");
     
}