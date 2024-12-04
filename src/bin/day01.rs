use std::collections::HashMap;

fn main() {
  // read file from day01.txt
  let input = std::fs::read_to_string("day01.txt").unwrap();
  // split the input by newlines
  let lines = input.lines();

  let mut list1: Vec<i32> = vec![];
  let mut list2: Vec<i32> = vec![];
  for line in lines {
    let nums: Vec<i32> = line.split("   ").map(|x| x.parse::<i32>().unwrap()).collect();
    list1.push(nums[0]);
    list2.push(nums[1]);
  }

  list1.sort();
  list2.sort();

  list1.reverse();
  list2.reverse();

  let mut frequency = HashMap::new();
  for num in list2.clone() {
    let count = frequency.entry(num).or_insert(0);
    *count += 1;
  }

  let mut distance = 0;
  let mut similarity = 0;
  for _ in 0..list1.len() {
    let l1 = list1.pop().unwrap();
    distance += (l1 - list2.pop().unwrap()).abs();
    similarity += l1 * frequency.get(&l1).unwrap_or(&0);
  }

  println!("Distance: {}", distance);
  println!("Similarity: {}", similarity);
}