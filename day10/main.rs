use std::fs::File;
use std::io::Read;
use std::env;
use std::collections::HashMap;


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
enum TileType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Tile {
    tile_type: TileType,
    position: (i32, i32),
}

impl Tile {
    fn can_go(&self, destination: Tile) -> bool{
        match self.tile_type {
            TileType::Vertical => {
                if destination.position.1 == self.position.1 +1{
                    match destination.tile_type {
                        TileType::Vertical => true,
                        TileType::Horizontal => false,
                        TileType::NorthEast => true,
                        TileType::NorthWest => true,
                        TileType::SouthWest => false,
                        TileType::SouthEast => false,
                        TileType::Ground => false,
                        TileType::Start => true,
                    }
                }else if destination.position.1 == self.position.1 -1{
                    match destination.tile_type {
                        TileType::Vertical => true,
                        TileType::Horizontal => false,
                        TileType::NorthEast => false,
                        TileType::NorthWest => false,
                        TileType::SouthWest => true,
                        TileType::SouthEast => true,
                        TileType::Ground => false,
                        TileType::Start => true,
                    }
                }else{
                    false
                }
            },

            TileType::Horizontal => {
                if destination.position.0 == self.position.0 +1 {
                    match destination.tile_type {
                        TileType::Vertical => false,
                        TileType::Horizontal => true,
                        TileType::NorthEast => false,
                        TileType::NorthWest => true,
                        TileType::SouthWest => true,
                        TileType::SouthEast => false,
                        TileType::Ground => false,
                        TileType::Start => true,
                    }
                }else if destination.position.0 == self.position.0 -1 {
                    match destination.tile_type {
                        TileType::Vertical => false,
                        TileType::Horizontal => true,
                        TileType::NorthEast => true,
                        TileType::NorthWest => false,
                        TileType::SouthWest => false,
                        TileType::SouthEast => true,
                        TileType::Ground => false,
                        TileType::Start => true,
                    }
                }else{
                    false
                }
            },

            TileType::NorthEast => {
                if destination.position.1 == self.position.1 -1 {
                    match destination.tile_type {
                        TileType::Vertical => true,
                        TileType::Horizontal => false,
                        TileType::NorthEast => false,
                        TileType::NorthWest => false,
                        TileType::SouthWest => true,
                        TileType::SouthEast => true,
                        TileType::Ground => false,
                        TileType::Start => true,
                    }
                }else if destination.position.0 == self.position.0 +1 {
                    match destination.tile_type {
                        TileType::Vertical => false,
                        TileType::Horizontal => true,
                        TileType::NorthEast => false,
                        TileType::NorthWest => true,
                        TileType::SouthWest => true,
                        TileType::SouthEast => false,
                        TileType::Ground => false,
                        TileType::Start => true,
                    }
                }else{
                    false
                }
            },

            TileType::NorthWest => {
                if destination.position.1 == self.position.1 -1 {
                    match destination.tile_type {
                        TileType::Vertical => true,
                        TileType::Horizontal => false,
                        TileType::NorthEast => false,
                        TileType::NorthWest => false,
                        TileType::SouthWest => true,
                        TileType::SouthEast => true,
                        TileType::Ground => false,
                        TileType::Start => true,
                    }
                }else if destination.position.0 == self.position.0 - 1 {
                    match destination.tile_type {
                        TileType::Vertical => false,
                        TileType::Horizontal => true,
                        TileType::NorthEast => true,
                        TileType::NorthWest => false,
                        TileType::SouthWest => false,
                        TileType::SouthEast => true,
                        TileType::Ground => false,
                        TileType::Start => true,
                    }
                }else {
                    false
                }
            },

            TileType::SouthEast => {
                if destination.position.1 == self.position.1 +1 {
                    match destination.tile_type {
                        TileType::Vertical => true,
                        TileType::Horizontal => false,
                        TileType::NorthEast => true,
                        TileType::NorthWest => true,
                        TileType::SouthWest => false,
                        TileType::SouthEast => false,
                        TileType::Ground => false,
                        TileType::Start => true,
                    }
                }else if destination.position.0 == self.position.0 +1 {
                    match destination.tile_type {
                        TileType::Vertical => false,
                        TileType::Horizontal => true,
                        TileType::NorthEast => false,
                        TileType::NorthWest => true,
                        TileType::SouthWest => true,
                        TileType::SouthEast => false,
                        TileType::Ground => false,
                        TileType::Start => true,
                    }
                }else {
                    false
                }
            },

            TileType::SouthWest => {
                if destination.position.0 == self.position.0 -1 {
                    match destination.tile_type {
                        TileType::Vertical => false,
                        TileType::Horizontal => true,
                        TileType::NorthEast => true,
                        TileType::NorthWest => false,
                        TileType::SouthWest => false,
                        TileType::SouthEast => true,
                        TileType::Ground => false,
                        TileType::Start => true,
                    }
                }else if destination.position.1 == self.position.1 +1 {
                    match destination.tile_type {
                        TileType::Vertical => true,
                        TileType::Horizontal => false,
                        TileType::NorthEast => true,
                        TileType::NorthWest => true,
                        TileType::SouthWest => false,
                        TileType::SouthEast => false,
                        TileType::Ground => false,
                        TileType::Start => true,
                    }
                }else {
                    false
                }
            },

            TileType::Ground => {
                false
            },

            TileType::Start => {
                if destination.position.0 == self.position.0 +1 {
                    match destination.tile_type {
                        TileType::Vertical => false,
                        TileType::Horizontal => true,
                        TileType::NorthEast => false,
                        TileType::NorthWest => true,
                        TileType::SouthWest => true,
                        TileType::SouthEast => false,
                        TileType::Ground => false,
                        TileType::Start => false,
                    }
                }else if destination.position.0 == self.position.0 -1 {
                    match destination.tile_type {
                        TileType::Vertical => false,
                        TileType::Horizontal => true,
                        TileType::NorthEast => true,
                        TileType::NorthWest => false,
                        TileType::SouthWest => false,
                        TileType::SouthEast => true,
                        TileType::Ground => false,
                        TileType::Start => false,
                    }
                }else if destination.position.1 == self.position.1 -1 {
                    match destination.tile_type {
                        TileType::Vertical => true,
                        TileType::Horizontal => false,
                        TileType::NorthEast => false,
                        TileType::NorthWest => false,
                        TileType::SouthWest => true,
                        TileType::SouthEast => true,
                        TileType::Ground => false,
                        TileType::Start => false,
                    }
                }else if destination.position.1 == self.position.1 +1 {
                    match destination.tile_type {
                        TileType::Vertical => true,
                        TileType::Horizontal => false,
                        TileType::NorthEast => true,
                        TileType::NorthWest => true,
                        TileType::SouthWest => false,
                        TileType::SouthEast => false,
                        TileType::Ground => false,
                        TileType::Start => false,
                    }
                }else {
                    false
                }
            },
        }
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

fn parse_file_content(file_content: &str) -> Vec<Vec<Tile>> {
    let lines: Vec<&str> = file_content.split("\n").collect();

    let mut grid: Vec<Vec<Tile>> = Vec::new();

    for (r_idx,line) in lines.iter().enumerate() {
        let mut row: Vec<Tile> = Vec::new();
        for (c_idx,node) in line.chars().enumerate() {
            if node != '\r' {
                match node {
                    '|' => {
                        let tile = Tile {
                            tile_type: TileType::Vertical,
                            position: (c_idx as i32, r_idx as i32),
                        };

                        row.push(tile);
                    },

                    '-' => {
                        let tile = Tile {
                            tile_type: TileType::Horizontal,
                            position: (c_idx as i32, r_idx as i32),
                        };

                        row.push(tile);
                    },

                    'L' => {
                        let tile = Tile {
                            tile_type: TileType::NorthEast,
                            position: (c_idx as i32, r_idx as i32),
                        };

                        row.push(tile);
                    },

                    'J' => {
                        let tile = Tile {
                            tile_type: TileType::NorthWest,
                            position: (c_idx as i32, r_idx as i32),
                        };

                        row.push(tile);
                    },

                    '7' => {
                        let tile = Tile {
                            tile_type: TileType::SouthWest,
                            position: (c_idx as i32, r_idx as i32),
                        };

                        row.push(tile);
                    },

                    'F' => {
                        let tile = Tile {
                            tile_type: TileType::SouthEast,
                            position: (c_idx as i32, r_idx as i32),
                        };

                        row.push(tile);
                    },

                    '.' => {
                        let tile = Tile {
                            tile_type: TileType::Ground,
                            position: (c_idx as i32, r_idx as i32),
                        };

                        row.push(tile);
                    },

                    'S' => {
                        let tile = Tile {
                            tile_type: TileType::Start,
                            position: (c_idx as i32, r_idx as i32),
                        };

                        row.push(tile);
                    },

                    _ => {}
                }
            }
        }

        grid.push(row);
    }

    grid
}

fn get_neigbors(tile: &Tile, grid: &Vec<Vec<Tile>>) -> Vec<Tile> {
    let mut neigbors: Vec<Tile> = Vec::new();

    if tile.position.1 +1 < grid.len() as i32 {
        if tile.can_go(grid[(tile.position.1 +1) as usize][tile.position.0 as usize]) {
            neigbors.push(grid[(tile.position.1 +1) as usize][tile.position.0 as usize]);
        }
    }
    if tile.position.1 -1 >= 0 {
        if tile.can_go(grid[(tile.position.1 -1) as usize][tile.position.0 as usize]) {
            neigbors.push(grid[(tile.position.1 -1) as usize][tile.position.0 as usize]);
        }
    }
    if tile.position.0 +1 < grid[0].len() as i32 {
        if tile.can_go(grid[(tile.position.1) as usize][(tile.position.0 +1) as usize]) {
            neigbors.push(grid[(tile.position.1) as usize][(tile.position.0 +1) as usize]);
        }
    }
    if tile.position.0 -1 >= 0 {
        if tile.can_go(grid[(tile.position.1) as usize][(tile.position.0 -1) as usize]) {
            neigbors.push(grid[(tile.position.1) as usize][(tile.position.0 -1) as usize]);
        }
    }
    
    neigbors
} 

fn find_starting_node(grid: &Vec<Vec<Tile>>) -> Option<Tile> {
    for (_r_idx,row) in grid.iter().enumerate() {
        for (_c_idx, node) in row.iter().enumerate(){
            if node.tile_type == TileType::Start {
                return Some(*node);
            }
        }
    }

    return None;
}

fn dfs(grid: &Vec<Vec<Tile>>, pos: Tile) -> (i32, HashMap<Tile, bool>){

    let mut stack: Vec<Tile> = Vec::new();

    let mut visited: HashMap<Tile, bool> = HashMap::new();

    stack.push(pos);

    let mut counter = 0;

    while stack.len() != 0 {
        let current = stack.pop().unwrap();

        if visited.get(&current) == None {
            counter += 1;
            visited.insert(current.clone(), true);
            for node in get_neigbors(&current, grid).iter() {
                if visited.get(node) == None {
                    stack.push(node.clone());    
                }
            }
        }
    }
    (counter/2 as i32, visited)
    
}

fn get_enclosed(grid: &Vec<Vec<Tile>>, visited: &HashMap<Tile, bool>) -> i32 {
    let mut counter = 0;
    for (r_idx,row) in grid.iter().enumerate() {

        
        for (c_idx, node) in row.iter().enumerate(){

            if visited.get(node) == None {

                let mut intersections = 0;
                for i in c_idx..row.len() {
                    if visited.get(&grid[r_idx][i]) != None {
                        let tile = grid[r_idx][i];
                        if vec![TileType::Vertical, TileType::SouthEast, TileType::SouthWest].contains(&tile.tile_type) {
                            intersections += 1
                        }
                        
                    }
                }

                if intersections % 2 == 1 {
                    print!("I");
                    counter += 1;
                    
                }else{
                    print!("O")
                }
            }else {
                print!("*");
            }
        }
        print!("\n");

        
    }

    counter
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

    let start = find_starting_node(&grid).expect("No start found");

    let (part1_solution, visited)  = dfs(&grid, start);

    let part2_solution = get_enclosed(&grid, &visited);

    println!("Part 1 Solution = {:?}" ,part1_solution);

    println!("Part 2 Solution = {:?}" ,part2_solution);




}