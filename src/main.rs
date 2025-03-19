use std::sync::atomic::Ordering;
use std::time::Duration;
use std::{
    sync::{Arc, atomic::AtomicBool},
    thread,
};

struct Timer {
    active: Arc<AtomicBool>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            active: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn set_timeout<F>(&self, function: F, delay: u64)
    where
        F: FnOnce() + Send + 'static,
    {
        let active = self.active.clone();

        thread::spawn(move || {
            if !active.load(Ordering::Relaxed) {
                return;
            }

            thread::sleep(Duration::from_millis(delay));

            if !active.load(Ordering::Relaxed) {
                return;
            }
            function();
        });
    }

    pub fn set_interval<F>(&self, function: F, interval_ms: u64)
    where
        F: Fn() + Send + 'static,
    {
        let active = self.active.clone();

        thread::spawn(move || {
            while active.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(interval_ms));
                if !active.load(Ordering::Relaxed) {
                    break;
                }

                function();
            }
        });
    }

    pub fn stop(&self) {
        self.active.store(false, Ordering::Relaxed);
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.stop();
    }
}

fn main() {
    let counter = Arc::new(std::sync::atomic::AtomicI32::new(0));
    let timer = Timer::new();
    let counter_clone = counter.clone();

    // timeout - only once
    timer.set_timeout(
        move || {
            println!("一次性定时器触发!");
            counter_clone.fetch_add(1, Ordering::SeqCst);
            println!("定时器计数:(来自一次性定时器){:?}", counter_clone);
        },
        200,
    );

    let counter_clone = counter.clone();
    // internal - repeat
    timer.set_interval(
        move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            println!("定时器计数: {:?}", counter_clone);
        },
        500,
    );

    // let main thread sleep for a while
    thread::sleep(Duration::from_millis(10000));

    // Stop all
    timer.stop();
    println!("所有定时器已停止");

    thread::sleep(Duration::from_millis(1000));

    println!("最终计数: {:?}", counter);
}
