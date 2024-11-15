/**
 * 这里考察的点在于:
 * 希望在 match 语句之后仍然能够使用 y, 使用 `ref` 或 `ref mut` 来借用 `y` 中的数据，
 * 而不是拿出数据(获取所有权)。通过这种方式，可以在不消耗 `y` 的情况下访问其内部数据。
 * 
 * 在 match 语句中使用 Some(ref p) 的要点:
 * 使用 `ref` 来借用而不是取得所有权。这样，`y`的所有权没有被消耗，可以在 match 之后继续访问。
 * ref 关键字使得 `p` 成为一个引用，访问 `p.x` 和 `p.y` 无需解引用，使得代码保持简单。
 */
struct Point{
    x: i32,
    y: i32,
}

fn main(){
    let y: Option<Point> = Some(Point{x: 10, y: 20});

    match y {
        // 编译器: value partially moved here
        Some(ref p) => println!("co-ordinate for p.x = {}, p.y = {}", p.x, p.y),
        _ => println!("Error, empty element!"),
    };
    y;
}