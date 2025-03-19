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
- 使用`thread::spawn`创建新线程执行定时任务
- 使用`Duration::from_millis`实现精确的时间控制
- 通过`Drop` trait实现自动清理资源

## 注意事项

- 计时器的精确度依赖于系统调度
- 所有计时器共享同一个停止开关
- 计时器停止后无法重新启动，需要创建新的计时器实例
