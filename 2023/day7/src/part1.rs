use std::collections::HashMap;
use std::io::prelude::*;

// hand types and their relative strengths
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)] 
enum EHandType {
    FiveKind = 7,
    FourKind = 6,
    FullHouse = 5,
    ThreeKind = 4,
    TwoPair = 3,
    OnePair = 2,
    High = 1,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand_type: EHandType,
    hand: String,
    bid: i64,
}


impl Hand {
    fn new(hand_type : EHandType, hand : &str, bid: i64) -> Self {
        Hand {
            hand_type,
            hand : hand.to_string(),
            bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { 

        // Obviously very inefficient to declare a hashmap witin a function in this way
        let mut char_to_int_map : HashMap<char,  i64> = HashMap::new();
        char_to_int_map.insert('T',10);
        char_to_int_map.insert('J',11);
        char_to_int_map.insert('Q',12);
        char_to_int_map.insert('K',13);
        char_to_int_map.insert('A',14);

        // Compare hand type hierarchies
        let hand_type_ordering = self.hand_type.partial_cmp(&other.hand_type);

        // If hand types are the same, compare hands
        hand_type_ordering.and_then(|hand_cmp| {
            if hand_cmp == std::cmp::Ordering::Equal {
                let mut comparison_result  = std::cmp::Ordering::Less;
                for i in 0..=4 {
                    let c = self.hand.chars().nth(i).unwrap();
                    let oc = other.hand.chars().nth(i).unwrap();
                    
                    let c_val : i64 = *char_to_int_map.get(&c).unwrap_or( &(c.to_digit(10).unwrap_or(0) as i64));
                    let oc_val : i64 = *char_to_int_map.get(&oc).unwrap_or(&(oc.to_digit(10).unwrap_or(0) as i64));
                    comparison_result = c_val.cmp(&oc_val);

                    if comparison_result != std::cmp::Ordering::Equal {
                        break;
                    }
                }
                Some(comparison_result)
            } else {
                Some(hand_cmp)
            }
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    // // Read file input
    let mut file = std::fs::File::open("input.txt")
        .expect("Unable to open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file!");

    let lines = contents.lines();

    let mut hands : Vec<Hand> = Vec::new();
    for line in lines {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        let hand_string = split_line[0];
        let hand_type = get_hand_type(hand_string);
        let bid = split_line[1].parse::<i64>().unwrap();

        let hand  = Hand {
            hand_type: hand_type,
            hand: hand_string.to_string(),
            bid: bid,
        };

        hands.push(hand)
    }

    // reverse sort
    hands.sort();
    let mut total_winnings : i64 = 0;
    for (index,hand) in hands.iter().enumerate() {
        // let enum_string = format!("{:?}", hand.hand_type);
        // println!("hand : {} {} {}", enum_string, hand.hand, hand.bid);
        // println!(" {} * {} ", (index as i64 +1), hand.bid);
        total_winnings += (index as i64 +1) * hand.bid;
    }
    println!("{}",total_winnings);
}

fn get_hand_type(hand: &str) -> EHandType {
    let mut freq_map : HashMap<char,  i64> = HashMap::new();
    
    for c in hand.chars() {
        let counter = freq_map.entry(c).or_insert(0);
        *counter += 1;
    }

    let has_five_kind = freq_map.values().any(|&value| value == 5);
    if has_five_kind {
        return EHandType::FiveKind;
    }
    let has_four_kind = freq_map.values().any(|&value| value == 4);
    if has_four_kind {
        return EHandType::FourKind;
    }
    let has_full_house = freq_map.values().any(|&value| value == 3) && freq_map.values().any(|&value| value == 2); 
    if has_full_house {
        return EHandType::FullHouse
    }
    let has_three_kind = freq_map.values().any(|&value| value == 3);
    if has_three_kind {
        return EHandType::ThreeKind;
    }
    let has_two_pair = freq_map.values().filter(|&&value| value == 2).count() == 2; 
    if has_two_pair {
        return EHandType::TwoPair;
    }
    let has_one_pair = freq_map.values().any(|&value| value == 2); 
    if has_one_pair {
        return EHandType::OnePair;
    }


    EHandType::High
}


fn verify_comparators() {
    let hand_five_k = Hand::new(EHandType::FiveKind, "AAAA9", 1);
    let hand_four_k = Hand::new(EHandType::FiveKind, "AAAAT", 1);
    let hand_full_house = Hand::new(EHandType::FullHouse, "AAAAB", 1);
    let hand_three_kind= Hand::new(EHandType::ThreeKind, "AAAAB", 1);
    let hand_two_pair= Hand::new(EHandType::TwoPair, "AAAAB", 1);
    let hand_one_pair = Hand::new(EHandType::OnePair, "AAAAB", 1);
    let hand_high = Hand::new(EHandType::High, "AAAAC", 1);

    assert!(hand_five_k < hand_four_k);
    assert!(hand_four_k > hand_full_house);
    assert!(hand_full_house > hand_three_kind);
    assert!(hand_three_kind > hand_two_pair);
    assert!(hand_two_pair > hand_one_pair);
    assert!(hand_one_pair > hand_high);
}