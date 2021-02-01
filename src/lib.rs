/// # fizzbuzz
///
/// Prints the fizzbuzz value of the given number:
/// * 'Fizz' if the number is divisible by 3
/// * 'Buzz' if the number is divisible by 5
/// * 'FizzBuzz' if the number is divisible by 15
/// * Prints the numerical value if not divisible by 3 or 5
/// ```
/// # use fizzbuzz::fizzbuzz;
/// assert_eq!(fizzbuzz(1), "1");
/// assert_eq!(fizzbuzz(3), "Fizz");
/// assert_eq!(fizzbuzz(5), "Buzz");
/// assert_eq!(fizzbuzz(15), "FizzBuzz");
/// ```
///
pub fn fizzbuzz(n: u32) -> String {
    match n {
        _ if n % 15 == 0 => "FizzBuzz".to_string(),
        _ if n % 5 == 0 => "Buzz".to_string(),
        _ if n % 3 == 0 => "Fizz".to_string(),
        _ => n.to_string(),
    }
}

#[cfg(test)]
mod tests;
