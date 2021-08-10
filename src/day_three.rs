use reqwest::header::COOKIE;

const PUZZLE_INPUT_URL: &str = "https://adventofcode.com/2020/day/3/input";

pub async fn solve() -> Result<(), reqwest::Error> {
    let mut input = puzzle_input().await.unwrap();
    input.pop();

    
    //println!("solving part one");
    //part_one(&input);
    println!("solving part two");
    part_two(&input);
    Ok(())
}

fn part_one(input: &Vec<String>) {
    let tree_count = tree_hit_calculation(input, 1, 3);
    println!("trees hit = {}", tree_count);
}

fn part_two(input: &Vec<String>) {
    let trees_hit: Vec<i128> = vec![
        tree_hit_calculation(input, 1, 1),
        tree_hit_calculation(input, 3, 1),
        tree_hit_calculation(input, 5, 1),
        tree_hit_calculation(input, 7, 1),
        tree_hit_calculation(input, 1, 2),
    ];
    println!("trees hit product = {}", trees_hit.iter().product::<i128>());
    println!("trees hit = {:?}", trees_hit);
}

fn tree_hit_calculation(input: &Vec<String>, horiz_shift: usize, vert_shift: usize) -> i128 {
    let mut tree_count = 0;
    let mut pointer = 0;
    let mut row_n = 0;
    for row in input {
        if row_n % vert_shift == 0 {
            let adjusted_point = pointer % row.len();
            let point = row.as_bytes()[adjusted_point];
            if point == b'#' {
                tree_count += 1;
            }
            pointer += horiz_shift;
        }

        row_n += 1;
    }

    tree_count
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
