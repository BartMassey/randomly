use randomly::*;

fn main() {
    let mut x = 0;
    for i in 1..=10 {
        print!("{i}: ");
        randomly! {
            { println!("show {x}"); }
            { x += 1; println!("increment"); }
            { println!("oops"); }
        }
    }
}
