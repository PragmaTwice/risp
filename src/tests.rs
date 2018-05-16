use super::*;

fn plus(x : i32, y : i32) -> i32 {
    x + y
}

#[test]
fn fuck() {
    assert_eq!(risp!{(plus 1 2)}, 3);
}