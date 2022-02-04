use randomly::*;

fn main() {
    let mut x = 0;
    for i in 1..=5 {
        randomly! {
            { println!("{i}: {x}"); }
            { x += 1; }
            { println!("oops"); }
        }
    }
}
