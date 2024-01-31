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

fn parse_file_content(file_content: &str) -> (HashMap<String, (String, String)>, String) {

    let lines: Vec<&str> =  file_content.split("\n").collect();

    let instructions: String = String::from(lines[0]);

    let mut node_map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines.iter().skip(2){

        let node_name = line.split("=").collect::<Vec<&str>>()[0].trim();

        let node_children = line.split("=").collect::<Vec<&str>>()[1].trim().trim_start_matches("(").trim_end_matches(")");

        let left_child = node_children.split(",").collect::<Vec<&str>>()[0].trim();
        let right_child = node_children.split(",").collect::<Vec<&str>>()[1].trim();

        let children = node_map.entry(String::from(node_name)).or_insert((String::from(left_child), String::from(right_child)));
        children.0 = String::from(left_child);
        children.1 = String::from(right_child);
        
    }

    (node_map, instructions)

}

fn get_num_steps(from_node: &str, to_node: &str, node_map: &HashMap<String, (String, String)> , instructions: &str) -> u64{

    let mut cyclic_iterator = instructions.chars().cycle();

    let mut src = from_node;

    let mut destination: &str = "";

    let mut num_steps = 0;

    while destination != to_node {
        let instruction = cyclic_iterator.next().expect("Failed to get next character in cycle");

        

        match instruction {
            'R' => {
                destination = &node_map.get(src).expect("Failed to get children").1[..];
                src = &destination;
                num_steps += 1;
                
            },

            'L' => {
                destination = &node_map.get(src).expect("Failed to get children").0[..];
                src = &destination;
                num_steps += 1;
            },
            _ => {},
        }

        
        
    }

    num_steps
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {return b;}
    return gcd(b % a, a);
}

fn lcm(list: Vec<u64>) -> u64 {

    return list.into_iter().reduce(|a, b| (a*b)/gcd(a,b)).unwrap();

}


fn get_num_steps2(node_map: &HashMap<String, (String, String)> , instructions: &str) -> u64 {

    let mut starting_nodes = node_map.keys().cloned()
                                        .filter(|s| s.ends_with('A'))
                                        .collect::<Vec<String>>();

    let mut steps_list: Vec<u64> = Vec::new();

    for mut node in &starting_nodes {
        
        let mut cyclic_iterator = instructions.chars().cycle();

        let mut src = node;

        let mut num_steps = 0;

        while !src.ends_with('Z') {

            let instruction = cyclic_iterator.next().expect("Failed to get next character in cycle");

            match instruction {
                'R' => {
                    src = &node_map.get(src).expect("Failed to get children").1;
                    num_steps += 1;
                    
                },

                'L' => {
                    src = &node_map.get(src).expect("Failed to get children").0;
                    num_steps += 1;
                },
                _ => {},
            }
        }

        steps_list.push(num_steps);
    }

    return lcm(steps_list);

    

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: main.exe <file_path>");
        panic!("No input file is provided.");
    }

    let file_path = &args[1];

    let file_content = read_input_file(file_path);

    let mut node_map: HashMap<String, (String, String)> = HashMap::new();
    let mut instructions: String = String::new();

    (node_map, instructions) = parse_file_content(&file_content[..]);

    let part1_solution = get_num_steps("AAA", "ZZZ", &node_map, &instructions[..]);

    let part2_solution = get_num_steps2(&node_map, &instructions);

    println!("Part 1 Solution = {}", part1_solution);

    println!("Part 2 Solution = {}", part2_solution);

}