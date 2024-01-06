use std::panic::catch_unwind;
use std::thread;
use std::time::Duration;

mod conc;
mod leetcode;

use crate::conc;
use crate::leetcode;

fn main() {
    println!("Hello, world!");

    // &str 和 String 的区别：
    // &str 是一个引用类型，表示指向 str 值的引用
    // &str 是一个 immutable（不可变的）引用，因此一旦创建，就不能修改它所引用的字符串
    // &str 通常用于函数参数中，当不需要修改字符串时，可以使用 &str 来传递字符串
    // String 是一个可变的字符串类型，它是在堆上分配的，并且可以自由地改变它的内容
    // String 提供了一系列的字符串操作函数，可以对字符串进行修改、拼接等操作

    // 由于 &str 是一个引用类型，它具有以下优势：
    // - &str 可以避免不必要的字符串分配，因为字符串引用只是指向现有字符串的指针，而不是复制字符串
    // - &str 在性能上通常优于 String，因为它不需要在堆上分配内存
    // - &str 在 Rust 中是 Copy 类型，可以轻松地在函数参数之间传递字符串副本
    let len = leetcode::length_of_last_word(" hello world ".to_owned());
    println!("最后一个单词的长度为：{:?}", len);

    conc::thread_spawn().unwrap();
}
