pub fn fizzbuzz(n: u32) -> String {
    n.to_string()
}

#[cfg(test)]
mod tests {

    #[test]
    fn fizbuzz_1_returns_1_as_a_string() {
        assert_eq!("1", crate::fizzbuzz(1));
    }

    #[test]
    fn fizbuzz_2_returns_2_as_a_string() {
        assert_eq!("2", crate::fizzbuzz(2));
    }
}
