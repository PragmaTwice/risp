use super::*;

fn plus(x : i32, y : i32) -> i32 {
    x + y
}

#[test]
fn risp_unit_checker_test() {
    assert_eq!(risp!{1}, 1);
    assert_eq!(risp!{1.0}, 1.0);
    assert_eq!(risp!{true}, true);

    let a = 1;
    assert_eq!(risp!{a}, 1);
}

#[test]
fn risp_item_transfer_test() {
    assert_eq!(risp!{(plus 1 2)}, 3);
    assert_eq!(risp!{(plus 1 (plus 2 3))}, risp!{(plus (plus 1 3) 2)});
}