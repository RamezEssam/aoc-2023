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


fn expand_space(grid: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut expanded_rows: Vec<usize> = Vec::new();
    let mut expanded_cols: Vec<usize> = Vec::new();
    
    for (c_idx,_ch) in grid[0].iter().enumerate() {
        if grid.iter().map(|row| row[c_idx]).all(|x| x == '.') {
            expanded_cols.push(c_idx);
        }
    }
            
    for (r_idx, _row) in grid.iter().enumerate(){
        if grid[r_idx].iter().all(|ch| *ch == '.') {
            expanded_rows.push(r_idx);
        }
    }
    
    (expanded_rows, expanded_cols)
}

fn manhattan_dist(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    return (a.0 - b.0).abs() + (a.1-b.1).abs()
}


fn sum_of_shortest(grid: &Vec<Vec<char>>, expanded_rows: &Vec<usize>, expanded_cols: &Vec<usize>, expansion_factor: usize) -> i64 {
    let mut galaxies: HashMap<i64, (i64, i64)> = HashMap::new();
    let mut galaxy_counter = 1;
    for (r_idx,row) in grid.iter().enumerate() {
        for (c_idx, ch) in row.iter().enumerate() {
            if *ch == '#' {
                galaxies.insert(galaxy_counter, (c_idx as i64, r_idx as i64));
                galaxy_counter += 1;
            }
        }
    }
    let mut distances: Vec<i64> = Vec::new();

    let mut hashed:HashSet<(&i64, &i64)> = HashSet::new();

    for (g_id1, galaxy1) in &galaxies {
        let mut new_galaxy1 = galaxy1.clone();
        for (_idx,row) in expanded_rows.iter().enumerate(){
            if galaxy1.1 as usize > *row {
                new_galaxy1 = (new_galaxy1.0, new_galaxy1.1 +(expansion_factor-1) as i64);
            }
        }
        for (_idx, col) in expanded_cols.iter().enumerate() {
            if galaxy1.0 as usize > *col {
                new_galaxy1 = (new_galaxy1.0 + (expansion_factor-1) as i64, new_galaxy1.1);
            }
        }
        for (g_id2, galaxy2) in &galaxies {
            if g_id1 != g_id2 {
                hashed.insert((g_id1, g_id2));
                let mut new_galaxy2 = galaxy2.clone();
                for (_idx,row) in expanded_rows.iter().enumerate(){
                    if galaxy2.1 as usize > *row {
                        new_galaxy2 = (new_galaxy2.0, new_galaxy2.1 +(expansion_factor-1) as i64);
                    }
                }
                for (_idx, col) in expanded_cols.iter().enumerate() {
                    if galaxy2.0 as usize > *col {
                        new_galaxy2 = (new_galaxy2.0 + (expansion_factor-1) as i64, new_galaxy2.1);
                    }
                }
                let shortest_path = manhattan_dist(&new_galaxy1, &new_galaxy2);
                if hashed.get(&(g_id2, g_id1)) == None {
                    distances.push(shortest_path);
                }
                
            }
        }
    }
    let sum: i64 = distances.iter().sum();

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

    let (expanded_rows,expanded_cols) = expand_space(&grid);

    let part1_solution = sum_of_shortest(&grid, &expanded_rows, &expanded_cols, 2);

    let part2_solution = sum_of_shortest(&grid, &expanded_rows, &expanded_cols, 1000000);

    println!("Part 1 Solution = {:?}", part1_solution);
    println!("Part 2 Solution = {:?}", part2_solution);


}