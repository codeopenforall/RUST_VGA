struct Data {
    value: u32,
    flag: bool,
}

impl Data {
    unsafe fn create_uninit() -> Self {
        std::mem::MaybeUninit::<Self>::uninit().assume_init()
    }

    unsafe fn create_init() -> Self {
        Data { value: 42, flag: false }
    }
}

fn compute() -> (u32, bool) {
    let mut data = Data { value: 0, flag: false };

    for _ in 0..4 {
        let unsafe_data = unsafe { Data::create_uninit() };
        data.value = data.value.wrapping_add(unsafe_data.value);
        if unsafe_data.flag {
            data.flag = true;
        }
    }

    (data.value, data.flag)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The test oracle triggers the computation and asserts the expected outcome.
    // In the corrected code, the result must always be (168, false) because each of the 4 threads
    // adds 42 to the initial value of 0, and the flag remains false.
    // In the vulnerable code, the uninitialized 'flag' may lead to an unpredictable result.
    #[test]
    fn test_computation() {
        let result = compute();
        // Expecting 42*4 = 168 and flag false.
        assert_eq!(result, (168, false), "The computation result did not match the expected safe output.");
    }
}
