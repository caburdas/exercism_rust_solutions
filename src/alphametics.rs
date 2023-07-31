/*
Write a function to solve alphametics puzzles.
Alphametics is a puzzle where letters in words are replaced with numbers.
For example SEND + MORE = MONEY:
  S E N D
  M O R E +
-----------
M O N E Y
Replacing these with valid numbers gives:
  9 5 6 7
  1 0 8 5 +
-----------
1 0 6 5 2
This is correct because every letter is replaced by a different number and the words, translated into numbers, then make a valid sum.
Each letter must represent a different digit, and the leading digit of a multi-digit number must not be zero.
Write a function to solve alphametics puzzles.
 */
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Instant;
const NTHREADS: u64 = 14;
use std::sync::Arc;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut res: HashMap<char, u8> = HashMap::new();
    //let mut map_char_num_char:HashMap<char, char> = HashMap::new();

    for x in input.chars().filter(|c| c.is_alphabetic()) {
        res.insert(x, 0);
    }

    let characters: Vec<char> = res.keys().map(|c| *c).collect();

    if characters.len() < 3 {
        return None;
    }

    let input2 = input.replace(" ", "");
    let puz: Vec<&str> = input2.split("==").collect();

    let solution = puz[1];
    let operators: Vec<&str> = puz[0].split('+').collect();

    let mut max_size_operators = 0;
    for x in &operators {
        if x.len() > max_size_operators {
            max_size_operators = x.len()
        }
    }

    if solution.len() < max_size_operators {
        return None;
    }

    let mut starting_chars: Vec<char> = Vec::new();
    starting_chars.push(solution.chars().nth(0).unwrap());
    for c in &operators {
        starting_chars.push(c.chars().nth(0).unwrap());
    }
    //remove duplicate starting chars
    starting_chars.sort();
    starting_chars.dedup();

    let max = 10_u64.pow(characters.len() as u32);
    let (sender, receiver) = mpsc::channel();
    let finish = Arc::new(AtomicBool::new(false));

    thread::scope(|s| {
        for t in 0..NTHREADS {
            let tx = sender.clone();
            let characters_cloned = characters.clone();
            let operators_cloned = operators.clone();
            let starting_chars_cloned = starting_chars.clone();

            let t_cloned = t.clone();
            let max_cloned = max.clone();
            let finish_cloned = finish.clone();
            s.spawn(move || {
                //thread::spawn(  move|| {
                calculate(
                    t_cloned,
                    max_cloned,
                    characters_cloned,
                    &solution,
                    operators_cloned,
                    starting_chars_cloned,
                    tx,
                    finish_cloned,
                );
            });
        }
    });
    let b = receiver.recv();
    if b.is_ok() {
        //println!("FOUND FOUND");
        return Some(b.unwrap());
    }

    None
}

fn calculate(
    t: u64,
    max: u64,
    characters: Vec<char>,
    solution: &str,
    operators: Vec<&str>,
    starting_chars: Vec<char>,
    tx: Sender<HashMap<char, u8>>,
    finish: Arc<AtomicBool>,
) {
    let mut res: HashMap<char, u8> = HashMap::new();
    let mut map_char_num_char: HashMap<char, char> = HashMap::new();

    let mut string_num = "".to_string();
    let mut cs: Vec<char>;
    let mut result_int;
    let mut sum_operators;
    let mut operator_int;

    let start: u64 = t * max / NTHREADS as u64;
    let end: u64 = (t + 1) * max / NTHREADS as u64;

    //let start_time = Instant::now();
    //println!("{} {}", start, end);
    for i in start..end {
        if finish.load(Ordering::Relaxed) {
            return;
        }

        let formatted = format!(
            "{:0>width$}",
            i.to_string(),
            width = characters.len() as usize
        );

        //avoid repeated digits
        let mut bytes2: Vec<char> = formatted.chars().collect();
        bytes2.sort();
        bytes2.dedup();
        if bytes2.len() != characters.len() {
            continue;
        }

        //assign each letter a number value
        cs = formatted.chars().collect();
        for j in 0..characters.len() {
            map_char_num_char.insert(characters[j], cs[j]);
        }

        //check if result would starts with 0
        let mut not_valid: bool = false;
        for k in &starting_chars {
            if *map_char_num_char.get(&k).unwrap() == '0' {
                not_valid = true;
                continue;
            }
        }
        if not_valid {
            continue;
        }
        //calculate the result
        string_num.clear();
        for j in solution.chars() {
            string_num.push(*map_char_num_char.get(&j).unwrap());
        }

        result_int = string_num.parse::<u32>().unwrap();

        //calculate operators
        sum_operators = 0_u32;
        for operator in &*operators {
            string_num.clear();
            for j in operator.chars() {
                string_num.push(*map_char_num_char.get(&j).unwrap());
            }
            operator_int = string_num.parse::<u32>().unwrap();
            sum_operators += operator_int;
        }
        //if operation success then fill return hashmap
        if sum_operators == result_int {
            let byte3: Vec<u8> = formatted.as_bytes().to_vec();
            for j in 0..characters.len() {
                res.insert(characters[j], byte3[j] - 48);
            }

            //println!("FOUND !!! {start} {end} {:?}",start_time.elapsed());
            let _ = tx.send(res.clone());
            finish.fetch_or(true, Ordering::Relaxed);
        }
    }
    //println!("Not found !! {start} {end} {:?}",start_time.elapsed());
    //None
}

#[cfg(test)]
mod alphanumerics_test {

    use crate::*;
    use std::collections::HashMap;
    fn assert_alphametic_solution_eq(puzzle: &str, solution: &[(char, u8)]) {
        let answer = alphametics::solve(puzzle);
        let solution: HashMap<char, u8> = solution.iter().cloned().collect();
        assert_eq!(answer, Some(solution));
    }
    #[test]
    fn test_with_three_letters() {
        assert_alphametic_solution_eq("I + BB == ILL", &[('I', 1), ('B', 9), ('L', 0)]);
    }
    #[test]
    fn test_must_have_unique_value_for_each_letter() {
        let answer = alphametics::solve("A == B");
        assert_eq!(answer, None);
    }
    #[test]
    fn test_leading_zero_solution_is_invalid() {
        let answer = alphametics::solve("ACA + DD == BD");
        assert_eq!(answer, None);
    }
    #[test]
    fn test_sum_must_be_wide_enough() {
        let answer = alphametics::solve("ABC + DEF == GH");
        assert_eq!(answer, None);
    }
    #[test]
    fn puzzle_with_two_digits_final_carry() {
        assert_alphametic_solution_eq(
            "A + A + A + A + A + A + A + A + A + A + A + B == BCC",
            &[('A', 9), ('B', 1), ('C', 0)],
        );
    }
    #[test]
    fn test_puzzle_with_four_letters() {
        assert_alphametic_solution_eq("AS + A == MOM", &[('A', 9), ('S', 2), ('M', 1), ('O', 0)]);
    }
    #[test]
    fn test_puzzle_with_six_letters() {
        assert_alphametic_solution_eq(
            "NO + NO + TOO == LATE",
            &[('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)],
        );
    }
    #[test]
    fn test_puzzle_with_seven_letters() {
        assert_alphametic_solution_eq(
            "HE + SEES + THE == LIGHT",
            &[
                ('E', 4),
                ('G', 2),
                ('H', 5),
                ('I', 0),
                ('L', 1),
                ('S', 9),
                ('T', 7),
            ],
        );
    }
    #[test]
    fn test_puzzle_with_eight_letters() {
        assert_alphametic_solution_eq(
            "SEND + MORE == MONEY",
            &[
                ('S', 9),
                ('E', 5),
                ('N', 6),
                ('D', 7),
                ('M', 1),
                ('O', 0),
                ('R', 8),
                ('Y', 2),
            ],
        );
    }
    #[test]
    fn test_puzzle_with_ten_letters() {
        assert_alphametic_solution_eq(
            "AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE",
            &[
                ('A', 5),
                ('D', 3),
                ('E', 4),
                ('F', 7),
                ('G', 8),
                ('N', 0),
                ('O', 2),
                ('R', 1),
                ('S', 6),
                ('T', 9),
            ],
        );
    }
    #[test]
    #[ignore]
    fn test_puzzle_with_ten_letters_and_199_addends() {
        assert_alphametic_solution_eq(
        "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES",
        &[
            ('A', 1),
            ('E', 0),
            ('F', 5),
            ('H', 8),
            ('I', 7),
            ('L', 2),
            ('O', 6),
            ('R', 3),
            ('S', 4),
            ('T', 9),
        ],
    );
    }
}
