// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use lib::util::fastio::*;

fn main() {
    let mut io = IO::new();
    let n: usize = io.read();
    for _ in 0..n {
        let a: i64 = io.read();
        let b: i64 = io.read();
        io.write(a + b);
    }
}
