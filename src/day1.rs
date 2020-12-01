use std::{
  fs::File,
  io::{BufRead, BufReader},
};

pub fn get_the_two_whose_sum_is_equal_to2020(v: Vec<i64>) -> (i64, i64, i64) {
  for i in 0..v.len() {
    for j in 0..v.len() {
      for k in 0..v.len() {
        if i == j || k == i || k == j {
          continue;
        }
        if v[i] + v[j] + v[k] == 2020 {
          return (v[i], v[j], v[k]);
        }
      }
    }
  }
  panic!("no solution found");
}

pub fn create_vec_from_txt(path: &str) -> Vec<i64> {
  let file = File::open(path).unwrap();
  let reader = BufReader::new(file);
  let mut res: Vec<i64> = Vec::new();
  for line in reader.lines() {
    let i = line.unwrap().parse::<i64>().unwrap();
    res.push(i);
  }
  return res;
}
