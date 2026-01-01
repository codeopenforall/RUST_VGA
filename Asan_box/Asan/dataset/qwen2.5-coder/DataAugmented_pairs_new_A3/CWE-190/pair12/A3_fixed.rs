struct Engine;

trait CheckedOps {
    fn compute(&self, a: u64, b: u64) -> u64;
}

impl CheckedOps for Engine {
    fn compute(&self, a: u64, b: u64) -> u64 {
        a.checked_mul(b).expect("Multiplication overflow")
    }
}

fn run() -> u64 {
    let engine = Engine;
    engine.compute(3000, 3000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_computation() {
        let result = run();
        assert_eq!(result, 9_000_000_000, "The computed product is incorrect, indicating an arithmetic overflow vulnerability.");
    }
}