pub fn compute(n: u64) -> u64 {
    const MAX_THREADS: u64 = 1000;
    let count = if n > MAX_THREADS { MAX_THREADS } else { n };
    count
}

#[cfg(test)]
mod tests {
    use super::compute;

    #[test]
    fn test_resource_consumption() {
        let input: u64 = 1500;
        let result = compute(input);
        // The fixed version should return exactly MAX_THREADS (1000)
        // while the vulnerable version returns the full count (1500).
        assert_eq!(result, 1000, "Resource consumption is not properly limited");
    }
}