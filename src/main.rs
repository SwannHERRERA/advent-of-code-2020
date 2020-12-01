mod day1;

use day1::{create_vec_from_txt, get_the_two_whose_sum_is_equal_to2020};

fn main() {
  println!("Hello World !");
  let vec = create_vec_from_txt("file/day1.txt");
  let (x, y, z) = get_the_two_whose_sum_is_equal_to2020(vec);
  println!("{}", x * y * z);
}
