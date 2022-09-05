use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct UserId(u32);

impl UserId {
    pub fn new() -> UserId {
        static COUNTER: AtomicU32 = AtomicU32::new(1);
        UserId(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}
