use reqwest::header::COOKIE;
use regex::Regex;

const PUZZLE_INPUT_URL: &str = "https://adventofcode.com/2020/day/4/input";

pub async fn solve() -> Result<(), reqwest::Error> {
    let input = puzzle_input().await.unwrap();
    println!("solving part one");
    println!("part one = {}", part_one(&input));
    println!("solving part two");
    println!("part two = {}", part_two(&input));

    Ok(())
}

pub fn part_one(input: &Vec<String>) -> i32 {
    calculate_total(input, false)
}

pub fn part_two(input: &Vec<String>) -> i32 {
    calculate_total(input, true)
}

fn calculate_total(input: &Vec<String>, extra_validations: bool) -> i32 {
    let mut total = 0;
    let mut test_case = String::new();
    for row in input.iter() {
        if row.is_empty() {
            if is_valid(&test_case, extra_validations) {
                total += 1;
            }
            test_case = String::new();
        } else {
            test_case.push_str(" ");
            test_case.push_str(row);
            test_case = test_case.trim().to_string();
        }
    }

    if !test_case.is_empty() && is_valid(&test_case, extra_validations) {
        total += 1;
    }

    total
}

fn is_valid(test_case: &String, extra_validations: bool) -> bool {
    let entries: Vec<&str> = test_case.split(" ").collect();
    let mut prefixes: Vec<&str> = Vec::new();
    let mut expected_prefixes = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for entry in entries.iter() {
        let entry = entry.split(":").collect::<Vec<&str>>();
        if entry[0] != "cid" {
            if extra_validations && !valid_entry(&entry) {
                return false
            }

            prefixes.push(entry[0]);
        }
    }

    prefixes.sort();
    expected_prefixes.sort();

    expected_prefixes == prefixes
}

fn valid_entry(entry: &Vec<&str>) -> bool {
    match entry[0] {
        "byr" => {
            let year = entry[1]
                .parse::<i32>()
                .unwrap();
            year >= 1920 && year <= 2002
        },
        "iyr" => {
            let year = entry[1]
                .parse::<i32>()
                .unwrap();
            year >= 2010 && year <= 2020
        },
        "eyr" => {
            let year = entry[1]
                .parse::<i32>()
                .unwrap();
            year >= 2020 && year <= 2030
        },
        "hgt" => {
            if entry[1].strip_suffix("cm").unwrap_or(entry[1]) != entry[1] {
                let number = entry[1]
                    .strip_suffix("cm")
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                number >= 150 && number <= 193
            } else if entry[1].strip_suffix("in").unwrap_or(entry[1]) != entry[1] {
                let number = entry[1]
                    .strip_suffix("in")
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                number >= 59 && number <= 76
            } else {
                false
            }
        },
        "hcl" => {
            let re = Regex::new(r"#[a-f0-9]{6}").unwrap();
            re.is_match(entry[1])
        },
        "ecl" => {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth",].contains(&entry[1])
        },
        "pid" => {
            entry[1].len() == 9
        },
        _ => false
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: Vec<String> = vec![
            String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"),
            String::from("byr:1937 iyr:2017 cid:147 hgt:183cm"),
            String::from(""),
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"),
            String::from("hcl:#cfa07d byr:1929"),
            String::from(""),
            String::from("hcl:#ae17e1 iyr:2013"),
            String::from("eyr:2024"),
            String::from("ecl:brn pid:760753108 byr:1931"),
            String::from("hgt:179cm"),
            String::from(""),
            String::from("hcl:#cfa07d eyr:2025 pid:166559648"),
            String::from("iyr:2011 ecl:brn hgt:59in"),
        ];
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn it_returns_one() {
        let input: Vec<String> = vec![
            String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"),
            String::from("byr:1937 iyr:2017 cid:147 hgt:183cm"),
        ];
        assert_eq!(part_one(&input), 1);
    }

    #[test]
    fn it_returns_valid_passports() {
        let input: Vec<String> = vec![
            String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980"),
            String::from("hcl:#623a2f"),
            String::from(""),
            String::from("eyr:2029 ecl:blu cid:129 byr:1989"),
            String::from("iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"),
            String::from(""),
            String::from("hcl:#888785"),
            String::from("hgt:164cm byr:2001 iyr:2015 cid:88"),
            String::from("pid:545766238 ecl:hzl"),
            String::from("eyr:2022"),
            String::from(""),
            String::from("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
        ];
        assert_eq!(part_two(&input), 4);
    }

    #[test]
    fn it_doesnt_return_invalid_passports() {
        let input: Vec<String> = vec![
            String::from("eyr:1972 cid:100"),
            String::from("hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
            String::from(""),
            String::from("iyr:2019"),
            String::from("hcl:#602927 eyr:1967 hgt:170cm"),
            String::from("ecl:grn pid:012533040 byr:1946"),
            String::from(""),
            String::from("hcl:dab227 iyr:2012"),
            String::from("ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
            String::from(""),
            String::from("hgt:59cm ecl:zzz"),
            String::from("eyr:2038 hcl:74454a iyr:2023"),
            String::from("pid:3556412378 byr:2007"),
        ];
        assert_eq!(part_two(&input), 0);
    }
}
