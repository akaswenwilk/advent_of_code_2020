use reqwest::header::COOKIE;

const PUZZLE_INPUT_URL: &str = "https://adventofcode.com/2020/day/2/input";

pub async fn solve() -> Result<(), reqwest::Error> {
    let input = puzzle_input().await.unwrap();

    
    println!("solving part one");
    part_one(&input);
    println!("solving part two");
    part_two(&input);
    Ok(())
}

fn part_two(input: &Vec<String>) {
    let mut valid_count = 0;
    for test_case in input.iter() {
        if !test_case.is_empty() && valid_part_two(test_case) {
            valid_count += 1;
        }
    }

    println!("{} valid cases", valid_count);
}

fn valid_part_two(test_case: &str) -> bool {
    let (min, max, letter, password) = destructure_test_case(test_case);
    validate_part_two(min, max, letter, password)
}

fn validate_part_two(min: usize, max: usize, letter: &str, password: &str) -> bool {
    let mut password_letters: Vec<&str> = password
        .split("")
        .collect::<Vec<&str>>();
    password_letters.pop();
    let first_letter = password_letters[min];
    let second_letter = password_letters[max];
    (first_letter == letter || second_letter == letter) && first_letter != second_letter
}

fn part_one(input: &Vec<String>) {
    let mut valid_count = 0;
    for test_case in input.iter() {
        if !test_case.is_empty() && valid(test_case) {
            valid_count += 1;
        }
    }

    println!("{} valid cases", valid_count);
}

fn valid(test_case: &str) -> bool {
    let (min, max, letter, password) = destructure_test_case(test_case);
    validate(min, max, letter, password)
}

fn destructure_test_case(test_case: &str) -> (usize, usize, &str, &str) {
    let factors: Vec<&str> = test_case.split(' ').collect();
    let mut factors = factors.iter();
    let numbers: Vec<&str> = factors.next().unwrap().split('-').collect();
    let min: usize = numbers[0].parse::<usize>().unwrap();
    let max: usize = numbers[1].parse::<usize>().unwrap();
    let letter = factors.next().unwrap().strip_suffix(':').unwrap();
    let password = factors.next().unwrap();
    (min, max, letter, password)
}

fn validate(min: usize, max: usize, letter: &str, password: &str) -> bool {
    let matching_letters: Vec<&str> = password
        .split("")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|l| {
            match *l == letter {
                true => Some(*l),
                false => None,
            }
        })
        .collect();
    matching_letters.len() >= min && matching_letters.len() <= max
}

async fn puzzle_input<'a>() -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get(PUZZLE_INPUT_URL)
        .header(COOKIE, "session=53616c7465645f5faea6e84806b0521dd759cec671405e90f579ee598539604dc145624380509600e3f8b5d76f647eec")
        .send()
        .await?
        .text()
        .await?;
    let vec: Vec<String> = resp
        .split("\n")
        .map(|s| String::from(s))
        .collect();
    Ok(vec)
}
