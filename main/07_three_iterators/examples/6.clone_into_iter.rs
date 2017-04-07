// There are times when you want create a new value
// when iterating over your original value.
// You might first `clone()`:

fn main() {
    println!("## 先clone()，再into_iter()");
    {
        let x = vec!["Jill", "Jack", "Jane", "John"];

        // into_iter()获取了x.clone()的所有权
        let y_1 = x.clone().into_iter().collect::<Vec<_>>();
        let y_2 = x.clone().into_iter().collect::<Vec<_>>();

        println!("x = {:?}", x);
        println!("y_1 = {:?}", y_1);
        println!("y_2 = {:?}", y_2);
    }

    println!("\n## 先clone()，再iter()，存在临时的clone()");
    {
        let x = vec!["Jill", "Jack", "Jane", "John"];

        // iter()返回x.clone()中每个元素的借用，但是x.clone()是临时的，当前语句结束后就被销毁了
        // let y_1 = x.clone().iter().collect::<Vec<_>>();
        // let y_2 = x.clone().iter().collect::<Vec<_>>();

        println!("x = {:?}", x);
        // println!("y_1 = {:?}", y_1);
        // println!("y_2 = {:?}", y_2);
    }

    println!("\n## 先clone()，再iter()，clone()的生命周期足够长");
    {
        let x = vec!["Jill", "Jack", "Jane", "John"];

        let x_1 = x.clone();
        let x_2 = x.clone();

        let y_1 = x_1.iter().collect::<Vec<_>>();
        let y_2 = x_2.iter().collect::<Vec<_>>();

        println!("x = {:?}", x);
        println!("y_1 = {:?}", y_1);
        println!("y_2 = {:?}", y_2);
    }
}