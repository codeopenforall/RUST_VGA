const MAX_QUEUE: usize = 5000;

fn run(trigger: usize) -> usize {
    let mut queue = Vec::new();
    for _ in 0..trigger {
        if queue.len() < MAX_QUEUE {
            queue.push(());
        } else {
            break;
        }
    }
    queue.len()
}

#[cfg(test)]
mod tests {
    use super::run;

    const SAFE_LIMIT: usize = 5000;

    #[test]
    fn test_resource_limit() {
        let total = run(10000);
        assert!(total <= SAFE_LIMIT, "Resource consumption exceeded safe limit: {}", total);
    }
}