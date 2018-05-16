trace_macros!(false);

macro_rules! risp_unit_checker {
    ($unit:literal) => ($unit);
    ($unit:ident) => ($unit);
}

macro_rules! risp_item_transfer {
    {(define $a:ident $b:tt)} => { let $a = lisp!{$b}; };
    {(begin $($a:tt)*)} => {{$(lisp!{$a});*}};
    {(lambda ($($a:tt)*) $b:tt)} => {(|$($a),*|lisp!{$b})};
    
    {(if $cond:tt $t:tt)} => {if lisp!{$cond} {lisp!($t)}};
    {(if $cond:tt $t:tt $f:tt)} => {if lisp!{$cond} {lisp!($t)} else {lisp!($f)}};
    
    {(loop $l:tt)} => {loop {lisp!($l)};
    {(while $cond:tt $l:tt)} => {while lisp!{$cond} {lisp!($l)};
    
    {($fn:tt $($item:tt)*)} => {risp!($fn)($(risp!{$item}),*)};
}

macro_rules! risp {
    {($($item:tt)+)} => {risp_item_transfer!{($($item)+)}};

    {$unit:tt} => {risp_unit_checker!($unit)};
}