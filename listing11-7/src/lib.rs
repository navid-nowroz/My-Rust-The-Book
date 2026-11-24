pub fn add(a: u64) -> u64 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add(2);
        assert_eq!(result, 4);
    }
}
