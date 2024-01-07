use std::io::*;
use std::str::FromStr;

fn main() {
    let mut l = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];

    let q: usize = read();
    for _ in 0..q {
        let query: usize = read();
        let k: usize = read();
        if query == 0 {
            println!("{}", l[k]);
        } else {
            let u: usize = read();
            l[k] = u;
        }
    }
}

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}
