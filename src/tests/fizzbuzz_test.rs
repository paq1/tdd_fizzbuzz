use crate::fizzbuzz::fizzbuzz;

#[test]
fn test_print_number_when_number_is_not_multiple_of_5_or_3() {
    let number = 1;
    let res = fizzbuzz(&number);
    assert_eq!("1".to_string(), res)
}

#[test]
fn should_print_fizz_when_number_is_multiple_of_3() {
    let number = 3;
    let res = fizzbuzz(&number);
    assert_eq!("fizz".to_string(), res)
}

#[test]
fn should_print_buzz_when_number_is_multiple_of_5() {
    let number = 5;
    let res = fizzbuzz(&number);
    assert_eq!("buzz".to_string(), res)
}

#[test]
fn should_print_fizzbuzz_when_number_is_multiple_of_5_and_3() {
    let number = 15;
    let res = fizzbuzz(&number);
    assert_eq!("fizzbuzz".to_string(), res)
}