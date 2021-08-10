use reqwest::header::COOKIE;

const PUZZLE_INPUT_URL: &str = "https://adventofcode.com/2020/day/1/input";

pub async fn solve() -> Result<(), reqwest::Error> {
    let input = puzzle_input().await.unwrap();
    println!("Solving part one");
    part_one(&input);

    println!("Solving part two");
    part_two(&input);

    Ok(())
}

fn part_one(input: &Vec<i32>) {
    let mut iterator = input.iter();
    for i in 0..input.len() {
        if let Some(first_number) = iterator.next() {
            let second_iterator = input[i+1..].iter();
            for second_number in second_iterator {
                if first_number + second_number == 2020 {
                    println!("the answer is {}", first_number * second_number);
                }
            }
        }
    }
}

fn part_two(input: &Vec<i32>) {
    let mut iterator = input.iter();
    for i in 0..input.len() {
        if let Some(first_number) = iterator.next() {
            let mut second_iterator = input[i+1..].iter();
            for j in i+1..input.len() {
                if let Some(second_number) = second_iterator.next() {
                    let third_iterator = input[j+1..].iter();
                    for third_number in third_iterator {
                        if first_number + second_number + third_number == 2020 {
                            println!("the answer is {}", first_number * second_number * third_number);
                            return
                        }
                    }
                }
            }
        }
    }
}

async fn puzzle_input() -> Result<Vec<i32>, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get(PUZZLE_INPUT_URL)
        .header(COOKIE, "session=53616c7465645f5faea6e84806b0521dd759cec671405e90f579ee598539604dc145624380509600e3f8b5d76f647eec")
        .send()
        .await?
        .text()
        .await?;
    let vec: Vec<&str> = resp
        .split("\n")
        .collect();
    let parsed: Vec<i32> = vec
        .iter()
        .filter_map(|&s| s.parse().ok())
        .collect();
    Ok(parsed)
}
