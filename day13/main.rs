use std::fs::File;
use std::io::Read;
use std::env;
use std::iter::zip;

fn read_input_file(filepath: &str) -> String{
    let mut input_file = match File::open(filepath) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut file_content = String::new();

    input_file.read_to_string(&mut file_content).unwrap();

    file_content
}

fn parse_file_content(file_content: &str) -> Vec<Vec<Vec<char>>> {

    let lines: Vec<&str> = file_content.split("\n").collect();
    let mut input_raw: Vec<Vec<char>> = Vec::new();

    let mut input: Vec<Vec<Vec<char>>> = Vec::new();
    let mut offset_idx: usize = 0;
    for  line in lines.into_iter() {
        let mut row: Vec<char> = Vec::new();

        for ch in line.chars() {
            if ch == '.' || ch == '#' {
                row.push(ch);
            } 
        }

        if line == "\r" {
            input.push(input_raw[offset_idx..].to_vec());
            offset_idx = input_raw.len();
        }else{
            input_raw.push(row);
        }

    }
    input.push(input_raw[offset_idx..].to_vec());

    input
    
}

fn part1(patterns: Vec<Vec<Vec<char>>>) -> usize {
    let mut total: usize = 0;
    for (grid_idx,grid) in patterns.iter().enumerate() {
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        let mut h_reflection_line: (usize, usize) = (0,0);
        let mut v_reflection_line: (usize, usize) = (0,0);

        // Horizontal reflection line 
        'outer: for (r_idx, _row) in grid.iter().enumerate() {
            if r_idx + 1 < num_rows {
                if zip(grid[r_idx].clone(), grid[r_idx+1].clone()).all(|(a,b)| a==b) {

                    'inner: for (r1_idx, r2_idx) in zip((0..=r_idx).rev(), ((r_idx+1)..num_rows).collect::<Vec<usize>>()) {
                        let r1 = &grid[r1_idx];
                        let r2 = &grid[r2_idx];
            
                        if zip(r1, r2).all(|(a,b)| *a==*b)  {
                            if r1_idx == 0 || r2_idx == num_rows-1 {
                                h_reflection_line = (r_idx, r_idx+1);
                                break 'outer;
                                
                            }
                        }else {
                            h_reflection_line = (0,0);
                            break 'inner;
                        }
                    }
                    
                }
            }
        }

    
        // Vertical reflection line 
        'outer: for c_idx in 0..num_cols {
            if c_idx+1 < num_cols {
                let c1 = grid.iter().map(|row| row[c_idx]).collect::<Vec<char>>(); 
                let c2 = grid.iter().map(|row| row[c_idx+1]).collect::<Vec<char>>();
                
                if zip(c1, c2).all(|(a,b)| a==b) {
                    'inner: for (c1_idx, c2_idx) in zip((0..=c_idx).rev(), ((c_idx+1)..num_cols).collect::<Vec<usize>>()) {
                        let c1 = grid.iter().map(|row| row[c1_idx]).collect::<Vec<char>>();
                        let c2 = grid.iter().map(|row| row[c2_idx]).collect::<Vec<char>>();
            
                        if zip(c1, c2).all(|(a,b)| a==b) {
                            if c1_idx == 0 || c2_idx == num_cols-1 {
                                v_reflection_line = (c_idx, c_idx+1);
                                break 'outer;
                            }
            
                        }else {
                            v_reflection_line = (0,0);
                            break 'inner;

                        }   
                        
                    }   
                }
            }
        }


        if h_reflection_line != (0,0) {
            println!(" grid {:?} horizontal mirror is between row {:?} and row {:?}, score = {:?}", grid_idx+1, h_reflection_line.0+1, h_reflection_line.1+1, (h_reflection_line.0 +1)*100);
            total += (h_reflection_line.0 +1)*100;
        }

        if v_reflection_line != (0,0) {
            println!(" grid {:?} vertical mirror is between column {:?} and column {:?}, score = {:?}", grid_idx+1, v_reflection_line.0+1, v_reflection_line.1+1, v_reflection_line.0 +1);
            total += v_reflection_line.0 +1;
        }
 
        
    }

    total
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: main.exe <file_path>");
        panic!("No input file is provided.");
    }

    let file_path = &args[1];

    let file_content = read_input_file(file_path);

    let input = parse_file_content(&file_content);

    let part1_solution = part1(input);

    println!("Part 1 Solution = {:?}", part1_solution);
  

}