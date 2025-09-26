// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x; //解构 Option，更清晰地访问 Some 中的值
    }
    println!("{}", res);
}
