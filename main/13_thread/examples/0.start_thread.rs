use std::thread;
use std::time::Duration;

fn main() {
    {
        thread::spawn(|| {
            println!("I am a anonymous sub thread.");
        });
        println!("I am the main thread.");
    }

    {
        // JoinHandle类型：https://doc.rust-lang.org/std/thread/struct.JoinHandle.html
        let join_handle: thread::JoinHandle<_> = thread::spawn(|| {
            println!("I am a anonymous sub thread.");
        });
        // `join()` waits for the associated thread to finish.
        // https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join
        join_handle.join().unwrap();
        println!("I am a main thread.");
    }

    {
        // 为子线程指定更多的参数信息，那么在创建的时候，可以使用Builder模式
        let builder = thread::Builder::new();

        let join_handle = builder
            .name("thread_1".to_string())
            .stack_size(4 * 1024 * 1024)
            .spawn(|| {
                println!("enter `thread_1` thread.");
                thread::park();  // park()暂停当前线程，进入等待状态
                println!("resume `thread_1` thread");
            }).unwrap();

        println!("spawn the `thread_1`");
        thread::sleep(Duration::new(5, 0));  // sleep()当前线程等待一段时间后继续执行
        join_handle.thread().unpark();  // thread()恢复线程，unpark()恢复线程的执行
        join_handle.join().unwrap();  // join()等待子线程执行结束
        println!("`thread_1` finished");
    }
}