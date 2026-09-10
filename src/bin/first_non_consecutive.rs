// https://www.codewars.com/kata/58f8a3a27a5c28d92e000144/train/rust

fn main() {
  let consecutive  = [1,2,3];

  let result = bla(&consecutive);
  println!("{}", result.unwrap());
} 

fn bla(consecutive: &[i32]) -> Option<i32> {
    if consecutive.len() < 2 {
    panic!("Consecutive array must have, at least, two elements")
  }

  let mut count = 0;

  for element in consecutive {
      let next_element = count + 1;
      count += 1;

      if count <= consecutive.len() && *element == consecutive[next_element] {
        return Some(*element);
      }
  }

  return None;
}