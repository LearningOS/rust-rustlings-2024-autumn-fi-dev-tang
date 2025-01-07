# std::sync::Arc 相关的 info
来源是:
https://rustwiki.org/zh-CN/std/sync/struct.Arc.html

```rust
pub struct Arc<T>
where
    T: ?Sized,
```
Arc<T> 类型提供了在堆中分配的 T 类型值的共享所有权。
在 Arc 上调用 Clone 会生成一个新的 Arc 实例，该实例指向堆上与源 Arc 相同的分配，同时增加了引用计数。
当指向给定分配的最后一个 Arc 指针被销毁时，存储在该分配中的值(通常称为“内部值”)也将被丢弃。

## 线程安全
Arc<T> 使用原子操作进行引用计数，线程安全的。如果不共享线程之间的引用计数分配，可以使用 Rc<T> 降低开销。