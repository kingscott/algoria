fn main() {
    println!("{}", bisection(|x| { x*x*x + x - 1_f32 }, 0, 1));
}

fn bisection<F: Fn(f32) -> f32>(f: F, x: i32, y: i32) -> f32 {
    let mut a = x as f32;
    let mut b = y as f32;
    let mut r = 0_f32;

    while (b - a).abs() >= 1e-4 {
        r = (a + b) / 2_f32;

        if f(a) == 0_f32 {
            break;
        } else if f(r) * f(a) < 0_f32 {
            b = r;
        } else {
            a = r;
        }
    }

    r
}
