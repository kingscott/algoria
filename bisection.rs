use std::f32;

fn main() {
    println!("{}", bisection(|x| { x*x*x + 3*x - 1 }, 0, 1));                 // Going to return 1*1*1 + 3*1 - 1 = 3
}

fn bisection<F: FnOnce(f32) -> f32>(f: F, x: i32, y: i32) -> i32 {
    let mut a = x as f32;
    let mut b = y as f32;
    let mut r = 0 as f32;

    while (b - a).abs() >= 1e-4f32 {
        r = (a + b) / 2;

        if f(a) == 0f32 {
            break;
        } else if (f(r) * f(a)) < 0f32 {
            b = r;
        } else {
            a = r;
        }
    }

    r
}
