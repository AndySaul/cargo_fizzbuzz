pub fn fizzbuzz(n: u32) -> String {
    n.to_string()
}

#[cfg(test)]
mod tests {

    #[test]
    fn fizbuzz_1_prints_1() {
        assert_eq!("1", crate::fizzbuzz(1));
    }

    #[test]
    fn fizbuzz_2_prints_2() {
        assert_eq!("2", crate::fizzbuzz(2));
    }
}
