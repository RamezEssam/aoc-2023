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

fn parse_file_content(file_content: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = file_content.split("\n").collect();

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let mut row:  Vec<char> = Vec::new();
        for node in line.chars() {
            if node != '\r' {
                row.push(node);
            }
        }

        grid.push(row);
    }

    grid
}

fn get_neigbors(x: i64, y: i64, grid: &Vec<Vec<char>>) -> Vec<(i64, i64)> {

    let mut neigbors: Vec<(i64, i64)> = Vec::new();

    let num_rows = grid.len() as i64;

    let num_cols = grid[0].len() as i64;

    match grid[y as usize][x as usize] {
        '|' => {
            if y+1 < num_cols {neigbors.push((x, y+1));}
            
            if y-1 >= 0 {neigbors.push((x, y-1));}
        },

        '-' => {
            if x+1 < num_rows {neigbors.push((x+1, y));}
            if x-1 >= 0 {neigbors.push((x-1, y));}
        },

        'L' => {
            if y-1 >= 0 {neigbors.push((x, y-1));}
            if x+1 < num_rows {neigbors.push((x+1, y));}
        },

        'J' => {
            if y-1 >=0 {neigbors.push((x, y-1));}
            if x-1 >=0 {neigbors.push((x-1, y));}
        },

        '7' => {
            if y+1 < num_cols {neigbors.push((x, y+1));}
            if x-1 >=0 {neigbors.push((x-1, y));}
        },

        'F' => {
            if y+1 < num_cols {neigbors.push((x, y+1));}
            if x+1 < num_rows {neigbors.push((x+1, y));}
        },

        'S' => {
            if x+1 < num_rows {
                if grid[y as usize][(x+1) as usize] != '.' {neigbors.push((x+1, y));}
            }
            if x-1 >=0 {

                if grid[y as usize][(x-1) as usize] != '.' {neigbors.push((x-1, y));}
            }
            if y+1 < num_cols {
                if grid[(y+1) as usize][x as usize] != '.' {neigbors.push((x, y+1));}
            }
            if y-1 >= 0 {
                if grid[(y-1) as usize][x as usize] != '.' {neigbors.push((x, y-1));}
            }
        },

        _ => {},
    }

    neigbors
} 

fn find_starting_node(grid: &Vec<Vec<char>>, start: char) -> Option<(i64, i64)> {
    for (r_idx,row) in grid.iter().enumerate() {
        for (c_idx, node) in row.iter().enumerate(){
            if *node == start {
                return Some((r_idx as i64, c_idx as i64));
            }
        }
    }

    return None;
}

fn dfs(grid: &Vec<Vec<char>>, pos: (i64, i64)) -> i64{

    let mut stack: Vec<(i64, i64)> = Vec::new();

    let mut visited: HashMap<(i64, i64), bool> = HashMap::new();

    stack.push(pos);

    let mut counter = 0;

    while stack.len() != 0 {
        let current = stack.pop().unwrap();

        if visited.get(&current) == None {
            counter += 1;
            visited.insert(current.clone(), true);
            for node in get_neigbors(current.0 as i64, current.1 as i64, grid).iter() {
                if visited.get(node) == None {
                    stack.push(node.clone());
                }
            }
        }
    }
    counter/2 as i64
    
}





fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: main.exe <file_path>");
        panic!("No input file is provided.");
    }

    let file_path = &args[1];

    let file_content = read_input_file(file_path);

    let grid = parse_file_content(&file_content[..]);

    let start = find_starting_node(&grid, 'S').expect("No start found");

    let part1_solution  = dfs(&grid, start);

    print!("Part 1 Solution = {:?}" ,part1_solution);




}