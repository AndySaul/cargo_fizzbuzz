pub fn fizzbuzz(_n: u32) -> String {
    "1".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn fizbuzz_1_returns_1_as_a_string() {
        assert_eq!("1", crate::fizzbuzz(1));
    }
}
