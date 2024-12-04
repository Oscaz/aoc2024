use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("day03.txt").unwrap();
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don\'t\(\))").unwrap();
    let mut results: Vec<Vec<i32>> = vec![];
    let mut enabled = true;
    for caps in re.captures_iter(&input) {
        let extract = caps.get(0);
        let mul = extract.unwrap().as_str();
        if mul.eq("do()") {
            enabled = true;
            continue;
        } else if mul.eq("don't()") {
            enabled = false;
            continue;
        }
        if !enabled {
            continue;
        }

        let mul = mul.replace("mul(", "").replace(")", "");
        let mul: Vec<i32> = mul.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
        results.push(mul);
    }
    let mut result = 0;
    for mul in results {
        result += mul[0] * mul[1];
    }
    println!("Result: {}", result);
}