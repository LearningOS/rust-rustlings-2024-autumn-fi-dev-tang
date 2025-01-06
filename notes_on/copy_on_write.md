# 关于 rust 里面 Copy-on-Write 的使用

## 1. Cow 是智能指针，提供两种状态
```rust
enum Cow<'a, B>{
    Borrowed(&'a B),    // 借用数据
    Owned(B)            // 拥有数据
}
```
## 2. 主要特点
- 延迟克隆: 只在需要修改的时候才克隆数据
- 可以包装借用数据或拥有所有权
- 通过 .to_mut() 按需克隆

```rust
use std::borrow::Cow;

fn example(data: &[i32]) -> Cow<[i32]>{
    if needs_modification(data){
        // 数据需要修改时克隆
        let mut owned = data.to_vec();
        // 修改数据
        Cow::Owned(owned)
    }else{
        // 不需要修改的时候直接借用
        Cow::Borrowed(data)
    }
}
```
避免不必要的克隆，适合读多写少的场景