// 使用 Rust 标准库的thread模块
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

// 一个Table有一个Mutex的vector
// 一个互斥锁是一个控制并发的方法：一次只有一个线程能访问它的内容。
// 这正叉子需要拥有的特性
struct Table {
    forks: Vec<Mutex<()>>,
}

// 创建了一个struct来代表一个哲学家
// 需要一个名字。选择String类型作为名字，而不是&str
// 通常来说，处理一个拥有它自己数据的类型要比使用引用的数据来的简单
struct Philosopher {
    name: String,
    // 每个哲学家将拥有两把叉子：一个拿左手，一个拿右手
    // 用usize来表示叉子，因为它是你的 vector 的索引的类型
    // 这两个值将会是我们Table中的forks的索引。
    left: usize,
    right: usize,
}

// impl块让我们在Philosopher上定义 new 方法
impl Philosopher {
    // new 这个关联方法有一个参数 name，是&str类型，另一个字符串的引用
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        // Rust中几乎所有的东西都是一个表达式并返回一个值
        // 函数中最后的表达式是自动返回的
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    // 在 Rust 中，方法显式获取一个self参数
    // 所以eat()是一个方法，而new是一个关联函数，关联函数new()没有用到self
    fn eat(&self, table: &Table) {
        // 访问Table的叉子列表
        // 使用self.left（self.right）来访问特定索引位置的叉子
        // 访问索引位置的Mutex，并在其上调用lock()
        // 如果互斥锁目前正在被别人访问，方法将阻塞直到左手的叉子可用为止
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        //命名结果为_left和_right
        // 用下划线？因为我们并不打算在锁中"使用"这些值，仅仅想要获取它
        // Rust会警告我们从未使用这些值，但是通过使用下划线，告诉Rust这是我们意图做的，这样就不会产生警告

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(2000));

        println!("{} is done eating.", self.name);
        // 释放锁会自动在_left和_right离开作用域时发生
    }
}

fn main() {
    // 创建了一个新Table
    // 封装在一个Arc<T>中，"arc"代表"原子引用计数"，需要在多个线程间共享Table
    // 因为共享，引用计数会增长，而当每个线程结束，引用计数会减少
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ]
    });

    // 创建了一个Vec<T>，Vec<T>也叫做一个"vector"，是一个可增长的数组类型
    // 再用for循环遍历 vector，顺序获取每个哲学家的引用
    let philosophers = vec![
        // 需要传left和right的值给Philosopher的构造函数
        // 不过这里有另一个细节，并且是"非常"重要，从头到尾全是连续的
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    // 让哲学家并发的进餐，增加的循环
    // 引入了一个新的绑定，叫做handles
    // 将创建一些新的线程，并且会返回这些线程句柄来控制行为

    // handles 上 Vec<_> 显式注明类型，_ 是一个类型占位符
    // 含义是说"handles 是一些东西的 vector，不过Rust可以发现这些东西是什么类型的"
    // 哲学家列表上调用into_iter()，创建了一个迭代器来获取每个哲学家的所有权
    // 在迭代器上调用map，传入map一个闭包作为参数，按顺序在每个哲学家上调用这个闭包
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        // 并发出现的代码块
        // 传入thread::spawn一个闭包作为参数，在一个新线程执行这个闭包
        // 这个闭包需要一个额外的标记，move，来表明这个闭包将会获取它获取的值的所有权。主要指map函数的p变量

        // 调用table.clone()
        // Arc<T>的clone()方法用来增加引用计数，而当它离开作用域，减少计数
        let table = table.clone();
        
        thread::spawn(move || {
            p.eat(&table);
        })
        // 注意到thread::spawn调用最后无分号，这样它是一个表达式。这个区别是重要的，以便生成正确的返回值（线程的句柄）
    }).collect();
    // collect()将会把返回值（线程的句柄）放入一个某种类型的集合，所以需要在 handles 上使用Vec<T>注明的类型

    // 遍历这些句柄
    // 在其上调用join()，会阻塞执行直到线程完成执行
    // 这保证了在程序结束之前完成这些线程
    for h in handles {
        h.join().unwrap();
    }
}