use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input!{
        r: i64,
        x: i64,
        y: i64
    }

    let dist = ((x*x + y*y) as f64).sqrt();
    let dist2 = x*x + y*y;

    let mut ans = 0;

    if x*x + y*y >= r*r {
        ans = ((dist2 / (r*r)) as f64).sqrt() as i64;

        if dist - (r * ans) as f64 > 0.0 {
            ans += 1
        }
    }
    else {
        ans = 2
    }

    println!("{}",ans)

}
