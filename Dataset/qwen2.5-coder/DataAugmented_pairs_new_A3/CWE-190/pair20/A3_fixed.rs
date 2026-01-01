fn compute_final() -> u32 {
    let mut ptr = 0u32;
    let add = u32::MAX;

    match ptr.checked_add(add) {
        Some(sum) => ptr = sum,
        None => ptr = u32::MAX,
    }

    ptr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        let res = compute_final();
        assert_eq!(res, u32::MAX, "Test oracle failed: Expected saturated value of u32::MAX.");
    }
}