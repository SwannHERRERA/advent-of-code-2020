use day1_lib::day1;

use day1::get_the_two_whose_sum_is_equal_to2020;


#[test]
fn test_get_the_two_whose_sum_is_equal_to2020() {
  let expense_report = vec![1721, 979, 366, 299, 675, 1456];
  let result = get_the_two_whose_sum_is_equal_to2020(expense_report);
  assert_eq!(result, (1721, 299));
  assert_eq!(299 + 1721, 2020);
}
