use std::panic::catch_unwind;
use std::thread;
use std::time::Duration;

pub fn thread_spawn() -> Result<(), Box<dyn std::error::Error>> {
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

    // 打印日志可以用 println! 宏
    // 类似 Go 语言中的 fmt.Println()、fmt.Printf()
    // 还有一个 format! 宏，类似 Go 语言中的 fmt.Sprintf()
    println!("执行完毕");

    Ok(())
}
