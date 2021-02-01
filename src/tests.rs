#[test]
fn fizbuzz_1_prints_1() {
    assert_eq!("1", crate::fizzbuzz(1));
}

#[test]
fn fizbuzz_2_prints_2() {
    assert_eq!("2", crate::fizzbuzz(2));
}

#[test]
fn fizbuzz_multiples_of_3_print_fizz() {
    assert_eq!("Fizz", crate::fizzbuzz(3));
    assert_eq!("Fizz", crate::fizzbuzz(21));
    assert_eq!("Fizz", crate::fizzbuzz(666));
}

#[test]
fn fizbuzz_multiples_of_5_print_buzz() {
    assert_eq!("Buzz", crate::fizzbuzz(5));
    assert_eq!("Buzz", crate::fizzbuzz(35));
    assert_eq!("Buzz", crate::fizzbuzz(665));
}

#[test]
fn fizbuzz_multiples_of_15_print_fizzbuzz() {
    assert_eq!("FizzBuzz", crate::fizzbuzz(15));
    assert_eq!("FizzBuzz", crate::fizzbuzz(45));
    assert_eq!("FizzBuzz", crate::fizzbuzz(750));
}
