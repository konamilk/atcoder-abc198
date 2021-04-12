use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut n: Chars
    }

    let mut nprime = vec![];

    let mut flg = false;
    for i in 0..n.len(){
        let j = n.len()- i - 1;
        if n[j] != '0'{
            nprime.push(n[j]);
            flg = true
        }
        else {
            if flg == false{
                continue
            }
            nprime.push(n[j]);
        }
    }

    let mut ans = "Yes";

    for i in 0..(nprime.len()/2) {
        if nprime[i] != nprime[nprime.len()-i - 1] {
            ans = "No"
        }
    }

    println!("{}", ans)
    // println!("{:?}",nprime)

}
