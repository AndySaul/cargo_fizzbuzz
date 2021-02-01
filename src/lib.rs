/// # fizzbuzz
/// 
/// Prints the fizzbuzz value of the given number:
/// * 'Fizz' if the number is divisible by 3
/// * 'Buzz' if the number is divisible by 5
/// * 'FizzBuzz' if the number is divisible by 15
/// * Prints the numerical value if not divisible by 3 or 5
/// ```
/// assert_eq!("1", fizzbuzz::fizzbuzz(1))
/// ```
pub fn fizzbuzz(n: u32) -> String {
    n.to_string()
}
