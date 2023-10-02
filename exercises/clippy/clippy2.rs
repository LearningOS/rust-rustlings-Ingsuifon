// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut res = 42i32;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
