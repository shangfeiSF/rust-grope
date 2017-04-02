fn main() {
    {
        struct Test {
            value_1: i32,
            value_2: char,
        }

        // 简略写法中
        // where子句对应的生命周期关系都没有标示出来
        // 而是依赖于编译器的自动推导
        impl Test {
            fn get<F>(&self, f: F) -> &i32
                where F: Fn(&Test) -> &i32
            {
                f(self)
            }
        }

        let test = Test {
            value_1: 100,
            value_2: 'A',
        };
        let borrow_of_value_1 = test.get(|t| &t.value_1);
        println!("borrow_of_value_1 = {}", borrow_of_value_1);

        // let borrow_of_value_2 = test.get(|t| &t.value_2);
        // println!("borrow_of_value_2 = {}", borrow_of_value_2);
    }

    {
        struct Test {
            value_1: i32,
            value_2: char,
        }

        // 现在要把生命周期全部手动补充完整，应该怎么做？
        // 参数和返回值的生命周期关系，很容易写出来
        // 但是，泛型约束中的Fn的参数和返回值的生命周期应该怎么填写？
        // 统一为同一个生命周期 'a 行不行？
        impl Test {
            fn get<'a, F>(&'a self, f: F) -> &'a i32
                where F: Fn(&'a Test) -> &'a i32
            {
                f(self)
            }
        }

        let test = Test {
            value_1: 100,
            value_2: 'A',
        };
        let borrow_of_value_1 = test.get(|t| &t.value_1);
        println!("borrow_of_value_1 = {}", borrow_of_value_1);

        // let borrow_of_value_2 = test.get(|t| &t.value_2);
        // println!("borrow_of_value_2 = {}", borrow_of_value_2);
    }

    {
        struct Test {
            value_1: i32,
            value_2: char,
        }

        impl Test {
            fn get<'a, F>(&'a self, f: F) -> &'a i32
                where F: Fn(&'a Test) -> &'a i32
            {
                f(self)
            }
        }

        // 统一为同一个生命周期 'a 的弊端
        fn get_closure(t: &Test) -> &i32 {
            &t.value_1
        }

        let test = Test {
            value_1: 100,
            value_2: 'A',
        };
        let borrow_of_value_1 = test.get(get_closure);
        println!("borrow_of_value_1 = {}", borrow_of_value_1);

        // let borrow_of_value_2 = test.get(|t| &t.value_2);
        // println!("borrow_of_value_2 = {}", borrow_of_value_2);
    }
}
