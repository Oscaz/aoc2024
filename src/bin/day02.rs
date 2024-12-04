fn main() {
  let input = std::fs::read_to_string("day02.txt").unwrap();
  let input = input.lines();
  let input: Vec<Vec<i32>> = input.map(|line| line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect()).collect();

  let mut safe_count = 0;
  for levels in input.clone() {
    if is_safe(levels, false) {
      safe_count += 1;
    }
  }

  println!("Safe: {}", safe_count);
}

fn is_safe(levels: Vec<i32>, dampened: bool) -> bool {
  let mut last: Option<i32> = None;
  let mut direction: Option<bool> = None; // true for ascending
  let mut safe = true;
  let mut dampened = dampened;
  for level in levels.clone() {
    if last.is_none() {
      last = Some(level);
      continue;
    }
    {
      let last = last.unwrap();
      if direction.is_none() {
        direction = Some(level > last);
      }
      let direction = direction.unwrap();
      if direction && level <= last {
        safe = false;
      }
      if !direction && level >= last {
        safe = false;
      }
      let difference = (level - last).abs();
      if difference < 1 || difference > 3 {
        safe = false;
      }
    }
    if !safe && !dampened {
      for i in 0..levels.len() {
        let mut levels = levels.clone();
        levels.remove(i);
        if is_safe(levels, true) {
          return true;
        }
      }
      dampened = true;
    } else {
      last = Some(level);
    }
  }
  return safe
}