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