use std::fs::File;
use std::io::Read;
use std::env;
use std::collections::HashMap;



fn read_input_file(filepath: &str) -> String{
    let mut input_file = match File::open(filepath) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut file_content = String::new();

    input_file.read_to_string(&mut file_content).unwrap();

    file_content
}

fn parse_file_content(file_content: &str) -> Vec<(String, Vec<usize>)> {
    let lines: Vec<&str> = file_content.split("\n").collect();

    let mut input: Vec<(String, Vec<usize>)> = Vec::new();

    for line in lines {
        let condition_record: String = line.split(' ').collect::<Vec<&str>>()[0].to_string();
        let arrng_size: Vec<usize> = line.split(' ')
                                         .collect::<Vec<&str>>()[1]
                                         .trim_end()
                                         .split(',')
                                         .map(|x| x.parse::<usize>().unwrap())
                                         .collect();

        input.push((condition_record, arrng_size));
    }

    input

}

fn parse_file_content2(file_content: &str) -> Vec<(String, Vec<usize>)> {
    let lines: Vec<&str> = file_content.split("\n").collect();

    let mut input: Vec<(String, Vec<usize>)> = Vec::new();

    for line in lines {
        let mut condition_record: String = line.split(' ').map(|x| x.to_string()).collect::<Vec<String>>()[0].to_string();

        condition_record = std::iter::repeat(condition_record).take(5).collect::<Vec<String>>().join(&"?");
        
        let mut arrng_size: Vec<usize> = line.split(' ')
                                         .collect::<Vec<&str>>()[1]
                                         .trim_end()
                                         .split(',')
                                         .map(|x| x.parse::<usize>().unwrap())
                                         .collect();
        let arrng_size_len = arrng_size.len();

        arrng_size = arrng_size.iter().cycle().take(arrng_size_len*5).cloned().collect::<Vec<usize>>();


        input.push((condition_record, arrng_size));
    }

    input

}



fn count_arrangements(arrangement: &str, sizes: Vec<usize>, mut cache: &mut HashMap<String, usize>) -> usize {
    if arrangement == "" {
        if sizes.is_empty() {
            return 1;
        }else{
            return 0;
        }
    }

    if sizes.is_empty() {
        if arrangement.contains("#") {
            return 0;
        }else {
            return 1;
        }
    }

    let key = format!("{arrangement:?}{sizes:?}");

    if cache.get(&key) != None {
        return *cache.get(&key).unwrap();
    }

    let mut result = 0;

    if arrangement.chars().nth(0) == Some('.') || arrangement.chars().nth(0) == Some('?') {
            result += count_arrangements(&arrangement[1..], sizes.clone(), &mut cache);
        
    }

    if arrangement.chars().nth(0) == Some('#') || arrangement.chars().nth(0) == Some('?') {
        if sizes[0] <= arrangement.len() && !arrangement.get(..sizes[0]).unwrap().contains(".") && arrangement.chars().nth(sizes[0]) != Some('#') {
                result += count_arrangements(&arrangement.get((sizes[0] + 1)..).unwrap_or(""), sizes.get(1..).expect("should have numbers").to_vec(), &mut cache);

        }
    }

    cache.insert(key, result);
    result
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: main.exe <file_path>");
        panic!("No input file is provided.");
    }

    let file_path = &args[1];

    let file_content = read_input_file(file_path);

    let mut cache: HashMap<String, usize> = HashMap::new();

    let input_part_1 = parse_file_content(&file_content[..]);

    let mut total = 0;
    for (arrng, sizes) in input_part_1 {
        let arrangements = count_arrangements(&arrng[..], sizes, &mut cache);
        total += arrangements;
    }

    println!("Part 1 Solution = {:?}", total);

    let input_part_2 = parse_file_content2(&file_content[..]);

    let mut total = 0;

    for (arrng, sizes) in input_part_2 {
        
        let arrangements = count_arrangements(&arrng[..], sizes.to_vec(), &mut cache);
        total += arrangements;

    }

    println!("Part 2 Solution = {:?}", total);
  


}