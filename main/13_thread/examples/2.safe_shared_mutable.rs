use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    {
        // let mut data = vec![1, 2, 3];
        // for i in 0..3 {
        //     thread::spawn(move || { data[i] += 1; });
        // }
        // thread::sleep(Duration::from_millis(50));
    }

    {
        // let mut data = Arc::new(vec![1, 2, 3]);
        // for i in 0..3 {
        //     let data = data.clone();
        //     thread::spawn(move || {
        //         data[i] += 1;
        //     });
        // }
        // thread::sleep(Duration::from_millis(50));
    }

    {
        // data 是`安全(safe)+共享(shared)+可变(mutable)`数据
        let data = Arc::new(Mutex::new(vec![1, 2, 3]));

        for i in 0..3 {
            // `clone()` makes a clone of the Arc pointer.
            // This creates another pointer to the same inner value, increasing the strong reference count.
            let data = data.clone();
            thread::spawn(move || {
                // `lock()` acquires a mutex and block the current thread until it is able to do so.
                // This function will block the local thread until it is available to acquire the mutex.
                // Upon returning, the thread is the only thread with the mutex held.
                // An RAII(Resource Acquisition Is Initialization) guard is returned to allow scoped unlock of the lock.
                // When the guard goes out of scope, the mutex will be unlocked.
                let mut data = data.lock().unwrap();
                data[i] += 1;
                println!("data[{:?}] = {:?}", i, data[i]);
            });
        }

        thread::sleep(Duration::from_millis(50));
        print!("data = {:?}", data);
    }
}
