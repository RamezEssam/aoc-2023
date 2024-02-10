use std::fs::File;
use std::io::Read;
use std::env;
use std::collections::{HashMap, HashSet};



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
        let mut row: Vec<char> = Vec::new();
        for ch in line.chars() {
            if ch != '\r' {
                row.push(ch);
            }
        }
        grid.push(row);
    }

    grid

}


fn expand_space(grid: &Vec<Vec<char>>, expansion_factor: usize) -> Vec<Vec<char>> {
    let mut expanded_grid: Vec<Vec<char>> = Vec::new();
    for (r_idx,row) in grid.iter().enumerate() {
        let mut expanded_row: Vec<char> = Vec::new();
        for (c_idx, ch) in row.iter().enumerate() {
            if grid.iter().map(|row| row[c_idx]).all(|x| x == '.') {
                for _ in 0..expansion_factor {
                    expanded_row.push(ch.clone());
                }
        
            }else {
                expanded_row.push(ch.clone());
            }
            
        }
        if grid[r_idx].iter().all(|ch| *ch == '.') {
            for _ in 0..expansion_factor {
                expanded_grid.push(expanded_row.clone())
                }
        }else {
            expanded_grid.push(expanded_row.clone())
        }

    }
    expanded_grid
}

fn manhattan_dist(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    return (a.0 - b.0).abs() + (a.1-b.1).abs()
}


fn sum_of_shortest(grid: &Vec<Vec<char>>) -> i32 {
    let mut galaxies: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut galaxy_counter = 1;
    for (r_idx,row) in grid.iter().enumerate() {
        for (c_idx, ch) in row.iter().enumerate() {
           
            if *ch == '#' {
                galaxies.insert(galaxy_counter, (c_idx as i32, r_idx as i32));
                galaxy_counter += 1;
            }
        }
    }

    let mut distances: Vec<i32> = Vec::new();

    let mut hashed:HashSet<(&i32, &i32)> = HashSet::new();

    for (g_id1, galaxy1) in &galaxies {
        for (g_id2, galaxy2) in &galaxies {
            if g_id1 != g_id2 {
                hashed.insert((g_id1, g_id2));
                let shortest_path = manhattan_dist(galaxy1, galaxy2);
                if hashed.get(&(g_id2, g_id1)) == None {
                    distances.push(shortest_path);
                }
                
            }
        }
    }
    let sum: i32 = distances.iter().sum();

    sum
    
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: main.exe <file_path>");
        panic!("No input file is provided.");
    }

    let file_path = &args[1];

    let file_content = read_input_file(file_path);

    let  grid = parse_file_content(&file_content[..]);

    let expanded_grid1 = expand_space(&grid, 2);

    let part1_solution = sum_of_shortest(&expanded_grid1);


    println!("Part 1 Solution = {:?}", part1_solution);


}