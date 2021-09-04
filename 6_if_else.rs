//rustc 6_if_else.rs
fn main() {
    let n = 10;

    if n < 0 {
        print!("{} es negativo", n);
    } else if n > 0 {
        print!("{} es positivo", n);
    } else {
        print!("{} es cero", n);
    }
}
