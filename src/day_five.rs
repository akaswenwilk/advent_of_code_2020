use reqwest::header::COOKIE;
use regex::Regex;

const PUZZLE_INPUT_URL: &str = "https://adventofcode.com/2020/day/5/input";
const ROWS: i32 = 128;
const COLUMNS: i32 = 8;

pub async fn solve() -> Result<(), reqwest::Error> {
    let input = puzzle_input().await.unwrap();
    println!("solving part one");
    println!("{} is the highest id", part_one(&input));
    Ok(())
}

fn part_one(input: &Vec<String>) -> i32 {
    0
}

fn get_id_from_row(row: String) -> i32 {
    get_row(&row) * COLUMNS + get_column(&row)
}

fn get_row(row: &String) -> i32 {
    0
}

fn get_column(row: &String) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_id_from_row_test() {
        let row = String::from("BFFFBBFRRR");
        assert_eq!(567, get_id_from_row(row));
        let row = String::from("FFFBBBFRRR");
        assert_eq!(119, get_id_from_row(row));
    }

    #[test]
    fn 
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
