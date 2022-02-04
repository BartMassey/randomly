/*!
Macro to select a random block. This is useful in games,
where taking random actions is common.

# Examples

```
use randomly::randomly;

let n = randomly! {
    { println!("hello"); 0 }
    { println!("goodbye"); 1 }
};
```
*/

/// Given a list of blocks, randomly execute one of the blocks
/// with equal probability for each.
#[macro_export]
macro_rules! randomly {
    (@$n:expr, $cur:block $next:block $($rest:block)*) => {
        if thread_rng().gen_range(0..$n) == 0 {
            randomly!(@$n + 1, $next $($rest)*)
        } else {
            randomly!(@$n + 1, $cur $($rest)*)
        }
    };
    (@$_:expr, $cur:tt) => {
        $cur
    };
    ($arg:block $($args:block)*) => {{
        use rand::{thread_rng, Rng};
        randomly!(@2u32, $arg $($args)*)
    }};
}

#[test]
fn test_randomly() {
    // XXX This test will fail with probability 1/3**100. I
    // can live with that.
    let mut changed = false;
    let mut last = randomly! {
        { 1 }
        { 2 }
        { 3 }
    };
    for _ in 0..100 {
        let n = randomly! {
            { 1 }
            { 2 }
            { 3 }
        };
        assert!((1u8..=3).contains(&n));
        if n != last {
            changed = true;
            last = n;
        }
    }
    assert!(changed);
}
