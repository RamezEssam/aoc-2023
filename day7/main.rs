use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::env;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
enum Card {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard(Card, Card, Card, Card, Card),
    OnePair(Card, Card, Card, Card, Card),
    TwoPair(Card, Card, Card, Card, Card),
    ThreeofKind(Card, Card, Card, Card, Card),
    FullHouse(Card, Card, Card, Card, Card),
    FourOfKind(Card, Card, Card, Card, Card),
    FiveOfKind(Card, Card, Card, Card, Card),  
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

fn parse_file_content(file_content: &str) -> Vec<(Hand, u64)> {

    let lines: Vec<&str> =  file_content.split("\n").collect();

    let mut hands: Vec<(Hand, u64)> = Vec::new();

    for line in lines {
        let parsed_line: Vec<&str> = line.split(" ").collect();

        let hand = parsed_line[0];
        let bid: u64 = parsed_line[1].trim().parse::<u64>().unwrap();

        let mut card_map: HashMap<Card, u8> = HashMap::new();
        let mut card_list: Vec<Card> = Vec::new();

        for card in hand.chars(){
            match card {
                '2' => {let count = card_map.entry(Card::Two).or_insert(0); *count += 1; card_list.push(Card::Two);},
                '3' => {let count = card_map.entry(Card::Three).or_insert(0); *count += 1; card_list.push(Card::Three);},
                '4' => {let count = card_map.entry(Card::Four).or_insert(0); *count += 1; card_list.push(Card::Four);},
                '5' => {let count = card_map.entry(Card::Five).or_insert(0); *count += 1; card_list.push(Card::Five);},
                '6' => {let count = card_map.entry(Card::Six).or_insert(0); *count += 1; card_list.push(Card::Six);},
                '7' => {let count = card_map.entry(Card::Seven).or_insert(0); *count += 1; card_list.push(Card::Seven);},
                '8' => {let count = card_map.entry(Card::Eight).or_insert(0); *count += 1; card_list.push(Card::Eight);},
                '9' => {let count = card_map.entry(Card::Nine).or_insert(0); *count += 1; card_list.push(Card::Nine);},
                'T' => {let count = card_map.entry(Card::Ten).or_insert(0); *count += 1; card_list.push(Card::Ten);},
                'J' => {let count = card_map.entry(Card::Jack).or_insert(0); *count += 1; card_list.push(Card::Jack);},
                'Q' => {let count = card_map.entry(Card::Queen).or_insert(0); *count += 1; card_list.push(Card::Queen);},
                'K' => {let count = card_map.entry(Card::King).or_insert(0); *count += 1; card_list.push(Card::King);},
                'A' => {let count = card_map.entry(Card::Ace).or_insert(0); *count += 1; card_list.push(Card::Ace);},
                _ => continue,
            }
            
        }

        let card_map_len = card_map.len();

        if card_map_len == 1 {
            hands.push((Hand::FiveOfKind(card_list[0], card_list[1], card_list[2], card_list[3], card_list[4]), bid));

        }else if card_map_len == 2{
            let max_count = *card_map.values().cloned().collect::<Vec<u8>>().iter().max().unwrap();

            if max_count == 3 {
                hands.push((Hand::FullHouse(card_list[0], card_list[1], card_list[2], card_list[3], card_list[4]), bid));
            
            }else {
            hands.push((Hand::FourOfKind(card_list[0], card_list[1], card_list[2], card_list[3], card_list[4]), bid));
            }

        }else if card_map_len == 3 {
            let max_count = *card_map.values().cloned().collect::<Vec<u8>>().iter().max().unwrap();
            if max_count == 3 {
                hands.push((Hand::ThreeofKind(card_list[0], card_list[1], card_list[2], card_list[3], card_list[4]), bid));
            } else if max_count == 2{
                hands.push((Hand::TwoPair(card_list[0], card_list[1], card_list[2], card_list[3], card_list[4]), bid));
            }
        }else if card_map_len == 4 {
            hands.push((Hand::OnePair(card_list[0], card_list[1], card_list[2], card_list[3], card_list[4]), bid));
        
        }else {
            hands.push((Hand::HighCard(card_list[0], card_list[1], card_list[2], card_list[3], card_list[4]), bid));
        }
    }

    hands
}


fn part1(hands: &mut Vec<(Hand, u64)>) -> u64 {

    hands.sort_by(|a,b| a.0.cmp(&b.0));

    let mut total_winnings: u64 = 0;

    for (i, (_hand, bid)) in hands.iter().enumerate() {
        total_winnings = total_winnings + (*bid * (i as u64+1));
    }

    total_winnings
}


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: main.exe <file_path>");
        panic!("No input file is provided.");
    }

    let file_path = &args[1];

    let file_content = read_input_file(file_path);

    let mut hands: Vec<(Hand, u64)> = parse_file_content(&file_content[..]);

    let part1_solution = part1(&mut hands);

    println!("Part1 Solution = {part1_solution}");
    
}