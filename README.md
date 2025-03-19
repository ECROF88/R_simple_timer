# R_simple_timer

一个用Rust实现的简单计时器库。提供类似JavaScript中`setTimeout`和`setInterval`的计时器功能。

## 特性

- ⚡ **一次性计时器** - 通过`set_timeout`实现延迟执行
- 🔄 **重复计时器** - 通过`set_interval`实现定期重复执行  
- 🛑 **优雅停止** - 支持随时停止所有计时器
- 💪 **并发安全** - 使用`Arc`和`AtomicBool`确保线程安全
- 🎯 **自动清理** - 实现`Drop` trait自动停止计时器

## 使用示例

```rust
use std::sync::Arc;
use std::time::Duration;
use std::thread;

fn main() {
    let counter = Arc::new(std::sync::atomic::AtomicI32::new(0));
    let timer = Timer::new();
    
    // 创建一个一次性计时器
    timer.set_timeout(
        || println!("一次性计时器触发!"),
        200  // 200ms后执行
    );
    
    // 创建一个重复计时器
    timer.set_interval(
        || println!("重复计时器触发!"),
        500  // 每500ms执行一次
    );
    
    // 让主线程等待一段时间
    thread::sleep(Duration::from_millis(2000));
    
    // 停止所有计时器
    timer.stop();
}
```

## 实现细节

- 使用`AtomicBool`控制计时器的活动状态

### 最初版本的实现：没有使用Arc

```rust
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::{
    sync::{Arc, atomic::AtomicBool},
    thread,
};
struct Timer {
    // active: Arc<AtomicBool>,
    active: &'static AtomicBool,
}

impl Timer {
    pub fn new() -> Self {
        let active = Box::leak(Box::new(AtomicBool::new(true)));
        Timer { active }
    }

    pub fn set_timeout<F>(&self, function: F, delay: u64)
    where
        F: Fn() + Send + 'static,
    {
        // let active = self.active.clone();
        let active = self.active;
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
        let active = self.active;
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
    println!("stop all timer");

    thread::sleep(Duration::from_millis(1000));

    println!("counter is: {:?}", counter);
}
```