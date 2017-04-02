//导函数
const DX: f64 = 0.000001; //微分函数的分母，尽可能的小，但由于浮点数的精度限制，取得太小，反而误差会扩大

fn deriv<'a, F>(f: &'a F) -> Box<Fn(f64) -> f64 + 'a>
    where F: Fn(f64) -> f64
{
    Box::new(move |x| (f(x + DX) - f(x - DX)) / (2.0 * DX))
}

//牛顿迭代法近似
fn newton<'a, F>(f: &'a F) -> Box<Fn(f64) -> f64 + 'a>
    where F: Fn(f64) -> f64
{
    Box::new(move |x| x - (f(x) / deriv(f)(x)))
}

//函数不动点逼近
fn fixed_point<'a, F, T>(fnt: &'a F, trans: T, first_guess: f64) -> f64
    where F: Fn(f64) -> f64,
          T: Fn(&'a F) -> Box<Fn(f64) -> f64 + 'a>
{
    let tolerance = 0.0000001;
    let close_enough = |a: f64, b: f64| (a - b).abs() > tolerance;
    let f = trans(fnt);
    let mut guess = first_guess;
    let mut next: f64 = f(guess);
    while close_enough(guess, next) {
        // println!("guess next:{}", next);
        guess = next;
        next = f(next);
    }
    next
}

fn sqrt(x: f64) -> f64 {
    let func = |y: f64| y.powi(2) - x;
    fixed_point(&func, newton, 1.0)
}

fn main() {
    let x = 2.0;
    let func = |y: f64| y.powi(2) - x;
    let dx = deriv(&func);
    println!("derivative y^2-x:{}", dx(1.0));
    let y = newton;
    let nt = y(&func);
    println!("newton method:{}", nt(1.4142));
    let val = fixed_point(&func, newton, 1.0);
    println!("fixed point,sqrt:{}", val);
    let x = 3.0;
    println!("sqrt(x):{}={}", x, sqrt(x));
}
