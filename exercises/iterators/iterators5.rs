// iterators5.rs
//
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try not to use imperative loops (for, while).
// Only the two iterator methods (count_iterator and count_collection_iterator)
// need to be modified.
//
// Execute `rustlings hint iterators5` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

/*
这里希望使用 HashMap 的 iterator, 同时又强调了不能使用 for, while 等循环
对类型进行特别说明:
1. 首先传入的参数是 
map: &HashMap<String, Progress> 
2. 接着对 map 转换成 iter(), map.iter() 之后的类型是
Iter<'_, String, Progress>
具体迭代时产生的是 (&String, &Progress) 类型的元组
3. 之后的 find(|key, value|: &(&String, &Progress))
    所以元素的类型是 &&String, &&Progress 
    解引用的时候得到的是 **v = Progress
4. 使用 &v 相当于解引用一次, 相当于 &Progress 和 Progress 进行比较。
5. 对于 PartialEq 的 trait 来说，rust会自动处理引用的解引用。
解引用一次之后， &v 相当于 &Progress, 可以直接和 Progress 相比较。
*/
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // map is a hashmap with String keys and Progress values.
    // map = { "variables1": Complete, "from_str": None, ... }
    let map_iter: usize = map
                        .iter()    // 把 HashMap 转换成 iterator
                        .filter(|(_, v)| **v == value)
                        .count();
    map_iter
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

/*
这里的 &[HashMap<String, Progress>] 是 HashMap<String, Progress> 类型元素的数组。
想要考察切片 slice 上的 map 方法类型转换

现在给出的类型是: &[HashMap<String, Progress>]
1. iter() 方法，转换成 
Iterator<&HashMap<String, Progress>>
2. map() 方法，转换每个 HashMap 为其值
Iterator<Value<String, Progress>>
3. flat_map() 方法，展平每个 Hashmap 的值
Iterator<&Progress>

4. flat_map(|map|map.values())
接受一个参数 map, map 是对 HashMap<String, Progress> 的不可变引用。
闭包返回的是 Values<'_, String, Progress> 类型的迭代器，迭代 Progress 值。
*/
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // collection is a slice of hashmaps.
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]
    let collection_iter: usize = collection
    .iter()
    .flat_map(|map| map.values())
    .filter(|v| **v == value)
    .count();
    collection_iter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(1, count_iterator(&map, Progress::Some));
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(2, count_iterator(&map, Progress::None));
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state)
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(1, count_collection_iterator(&collection, Progress::Some));
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(4, count_collection_iterator(&collection, Progress::None));
    }

    #[test]
    fn count_collection_equals_for() {
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        let collection = get_vec_map();

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state)
            );
        }
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}