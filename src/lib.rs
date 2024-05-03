#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]

use once_cell::sync::Lazy;
use std::sync::atomic::AtomicU8;

thread_local! {
    pub static THREAD_REGISTRY: Lazy<ThreadRegistry> = Lazy::new(ThreadRegistry::new);
}

pub struct ThreadRegistry {
    value: AtomicU8,
}

impl ThreadRegistry {
    pub fn new() -> Self {
        ThreadRegistry {
            value: AtomicU8::new(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    #[test]
    fn it_works() {
        THREAD_REGISTRY.with(|registry| {
            registry.value.store(10, Ordering::Relaxed);
            assert_eq!(registry.value.load(Ordering::Relaxed), 10, "main thread");
        });

        let x = std::thread::spawn(|| {
            THREAD_REGISTRY.with(|registry| {
                registry.value.store(5, Ordering::Relaxed);
                assert_eq!(registry.value.load(Ordering::Relaxed), 5, "spawned thread");
            });
        });

        x.join().unwrap();

        THREAD_REGISTRY.with(|registry| {
            assert_eq!(registry.value.load(Ordering::Relaxed), 10, "main thread");
        });
    }
}
