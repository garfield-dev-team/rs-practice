use std::panic::catch_unwind;
use std::thread;
use std::time::Duration;

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
    let len = length_of_last_word(" hello world ".to_owned());
    println!("最后一个单词的长度为：{:?}", len);

    let handle = thread::spawn(|| {
        // 保证当前线程 sleep 指定的时间，会阻塞当前线程
        thread::sleep(Duration::from_millis(2000));

        // 如果只是想让出 CPU 时间片，可以不用设置 0，而是调用 yield_now 函数
        // 类似 Go 语言中的 runtime.Gosched()
        thread::yield_now();

        // 在 Rust 可以使用 catch_unwind 实现类似 try/catch 捕获 panic 的功能
        // 如果 panic 没有被捕获，那么线程就会退出，通过 JoinHandle 可以检查这个错误
        // 如果被捕获，外部的 handle 是检查不到这个 panic 的
        let result = catch_unwind(|| {
            println!("执行前");
            panic!("发生 Panic");
            println!("执行后");
        });

        // catch_unwind 返回 Result 类型，如何获取其中的值
        // 可以用 unwrap() 方法获取 Ok 的值
        // 但是如果值是一个 Err 则会直接 panic
        // result.unwrap();

        // 推荐用 match 模式显式处理 Err
        match result {
            Ok(_) => println!("无 Panic"),
            Err(_) => println!("捕获到 Panic"),
        }
    });

    // join() 的作用是等待子线程退出
    // join() 返回一个 Result 类型，可以用 unwrap() 拿到 Result 的值
    handle.join().unwrap();
    println!("执行完毕");
}

pub fn length_of_last_word(s: String) -> usize {
    let s = s.trim();

    let v: Vec<&str> = s.split(' ').collect();

    v.iter().for_each(|x| println!("{}", x));

    v.last().unwrap().len()
}
