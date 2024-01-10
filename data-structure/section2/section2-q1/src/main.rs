// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [i32; n],
    }
    println!("{}", a[k]);
    println!("{}", a[n - 1 - k]);
}
