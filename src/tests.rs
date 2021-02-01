use super::fizzbuzz;

#[test]
fn fizbuzz_1_prints_1() {
    assert_eq!("1", fizzbuzz(1));
}

#[test]
fn fizbuzz_2_prints_2() {
    assert_eq!("2", fizzbuzz(2));
}

#[test]
fn fizbuzz_multiples_of_3_print_fizz() {
    assert_eq!("Fizz", fizzbuzz(3));
    assert_eq!("Fizz", fizzbuzz(21));
    assert_eq!("Fizz", fizzbuzz(666));
}

#[test]
fn fizbuzz_multiples_of_5_print_buzz() {
    assert_eq!("Buzz", fizzbuzz(5));
    assert_eq!("Buzz", fizzbuzz(35));
    assert_eq!("Buzz", fizzbuzz(665));
}

#[test]
fn fizbuzz_multiples_of_15_print_fizzbuzz() {
    assert_eq!("FizzBuzz", fizzbuzz(15));
    assert_eq!("FizzBuzz", fizzbuzz(45));
    assert_eq!("FizzBuzz", fizzbuzz(750));
}

#[test]
fn zero_is_a_valid_input() {
    assert_eq!("FizzBuzz", fizzbuzz(0));
}

#[test]
fn negative_numbers_are_valid_inputs() {
    assert_eq!("-1", fizzbuzz(-1));
    assert_eq!("Fizz", fizzbuzz(-3));
    assert_eq!("Buzz", fizzbuzz(-5));
    assert_eq!("FizzBuzz", fizzbuzz(-15));
}
