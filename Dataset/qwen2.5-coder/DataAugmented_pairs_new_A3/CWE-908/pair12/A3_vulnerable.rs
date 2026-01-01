pub mod app {
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub struct Item {
        secret: usize,
        public: AtomicUsize,
    }

    impl Item {
        pub fn new(init: bool) -> Self {
            Item {
                secret: 42,
                public: AtomicUsize::new(0),
            }
        }

        pub fn sum(&self) -> usize {
            self.secret + self.public.load(Ordering::Relaxed)
        }
    }
}