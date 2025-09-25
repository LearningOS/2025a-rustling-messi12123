// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1 调用 `.iter()` 获取不可变引用的迭代器

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));  // TODO: Step 2 依次取出第二个元素
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));          // TODO: Step 3 继续迭代
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);                    // TODO: Step 4 迭代器已结束，返回 None
}
