#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // 在堆上分配point，并返回一个指向它的指针
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    //  栈分配 point
    let point_by_struct: Point = Point { x: 0.0, y: 0.0 };
    let point_by_fn_directly: Point = origin();

    // 栈分配 rectangle
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 },
    };

    println!("## 栈内存分配：");
    println!("`point_by_struct` 占用栈内存 {} 字节", mem::size_of_val(&point_by_struct));
    println!("`point_by_fn_directly` 占用栈内存 {} 字节", mem::size_of_val(&point_by_fn_directly));
    println!("`rectangle` 占用栈内存 {} 字节", mem::size_of_val(&rectangle));

    // 堆分配 boxed_point
    let boxed_point_by_struct: Box<Point> = Box::new(Point { x: 0.0, y: 0.0 });
    let boxed_point_by_fn_directly: Box<Point> = boxed_origin();
    let boxed_point_by_fn_indirectly: Box<Point> = Box::new(origin());

    // 堆分配 boxed_rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 },
    });

    // 双重间接装箱（Double indirection）
    let double_boxed_point_1: Box<Box<Point>> = Box::new(boxed_origin());
    let double_boxed_point_2: Box<Box<Point>> = Box::new(Box::new(origin()));
    let double_boxed_point_3: Box<Box<Point>> = Box::new(Box::new(Point { x: 0.0, y: 0.0 }));

    // box的大小 = 指针大小
    println!("\n## 堆内存分配：");
    println!("`boxed_point_by_struct` 占用堆内存 {} 字节", mem::size_of_val(&boxed_point_by_struct));
    println!("`boxed_point_by_fn_directly` 占用堆内存 {} 字节", mem::size_of_val(&boxed_point_by_fn_directly));
    println!("`boxed_point_by_fn_indirectly` 占用堆内存 {} 字节", mem::size_of_val(&boxed_point_by_fn_indirectly));
    println!("`boxed_rectangle` 占用堆内存 {} 字节", mem::size_of_val(&boxed_rectangle));

    println!("`double_boxed_point_1` 占用堆内存 {} 字节", mem::size_of_val(&double_boxed_point_1));
    println!("`double_boxed_point_2` 占用堆内存 {} 字节", mem::size_of_val(&double_boxed_point_2));
    println!("`double_boxed_point_3` 占用堆内存 {} 字节", mem::size_of_val(&double_boxed_point_3));

    // 将包含在boxed_*的数据复制到unboxed_*中
    let unboxed_point: Point = *boxed_point_by_struct;
    let unboxed_rectangle: Rectangle = *boxed_rectangle;
    let unboxed_double: Box<Point> = *double_boxed_point_1;
    let unboxed_unboxed_double: Point = **double_boxed_point_2;

    println!("\n## 从堆内存复制到栈内存：");
    println!("`unboxed_point` 占用栈内存 {} 字节", mem::size_of_val(&unboxed_point));
    println!("`unboxed_rectangle` 占用栈内存 {} 字节", mem::size_of_val(&unboxed_rectangle));
    println!("`unboxed_double` 占用栈内存 {} 字节", mem::size_of_val(&unboxed_double));
    println!("`unboxed_unboxed_double` 占用栈内存 {} 字节", mem::size_of_val(&unboxed_unboxed_double));
}