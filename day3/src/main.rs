use regex::Regex;

fn main() {
    part1_with_regex();
}

fn part1_with_regex() {
    let text = input();
    let re = Regex::new(r"mul\(\d{1,3},\s?\d{1,3}\)").unwrap();
    let matches: Vec<&str> = re.find_iter(&text).map(|c| c.as_str()).collect();

    let mut sum_of_mul = 0usize;
    for m in matches.iter() {
        sum_of_mul += parse_get_mul(m);
    }
    println!("Total: {}", sum_of_mul);
}

fn parse_get_mul(m: &str) -> usize {
    let cleaned = m.replace("mul(", "").replace(")", "");
    let splitted: Vec<&str> = cleaned.split(",").collect();
    let x = splitted[0].trim().parse::<usize>().unwrap();
    let y = splitted[1].trim().parse::<usize>().unwrap();

    y * x
}

fn input() -> String {
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))a".to_string()
}
