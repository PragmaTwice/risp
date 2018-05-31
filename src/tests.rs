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
    assert_eq!(risp!{(+ 1 2)}, 3);
    assert_eq!(risp!{(+ 1 (+ 2 3))}, risp!{(+ 1 2 3)});
    assert_eq!(risp!{((lambda (x y z) (+ (* x x) (* y y) (* z z))) (- 4 1) 4 (+ 4 1))},50);
    assert_eq!(risp!{(begin (let a 1) (let b 2) (+ a b))},3);
    assert_eq!(risp!{((lambda (x y) (if (< x y) x y)) 10 15)}, 10);
    assert_eq!(risp!{(begin (let (mut i) 0) (while (< i 10) (+= i 1)) i)}, 10);
    assert_eq!(risp!{(begin (let (mut i) 0) (loop (begin (if (! (< i 10)) (break)) (+= i 1))) i)}, 10);
    assert_eq!(risp!{(match (tuple 3 (+ 1 2)) (=> (tuple 2 x) x) (=> (tuple 3 x) (+ x 1)) (=> (tuple 4 x) (+ x 2)) (=> _ 0))}, 4);
    assert_eq!(risp!{((((currying_lambda (x y z) (+ x y z)) 1) 2) 3)}, 6);
    assert_eq!(risp!{((lambda (x) (begin (let a x) (+ a 1))) 2)}, 3);

}
