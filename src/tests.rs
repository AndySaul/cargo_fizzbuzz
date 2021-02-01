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
