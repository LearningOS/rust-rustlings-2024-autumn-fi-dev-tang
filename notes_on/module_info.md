# rustlings 40 
写到 40 题的时候遇到了不懂的概念 modules 模块的可见性

follow rust 圣经学习 Rust 模块的使用，这里创建一个库类型的 Package, 使用下列命令:
```shell
cargo new --lib restaurant
```
修改库类型的 Package, 将修改的代码放入 "src/lib.rs" 中。

模块可以嵌套，模块中可以定义各种 Rust 类型，例如函数、结构体、枚举、特征等，所有模块均定义在同一个文件中。

模块树跟计算机上文件系统目录树的相似之处。不仅仅是组织结构上的相似，连使用方式也很相似：每个文件都有自己的路径，
用户可以通过这些路径使用它们，在Rust 中，我们也通过路径的方式来引用模块。

### 路径引用模块
- 绝对路径：从包根开始，路径名以包名或者 crate 作为开头
- 相对路径：从当前模块开始，以 self, super 或当前模块的标识符作为开头。

