trace_macros!(false);

macro_rules! risp_unit_checker {
    ($unit:literal) => ($unit);
    ($unit:ident) => ($unit);
}

macro_rules! risp_item_transfer {
    {(define $a:ident $b:tt)} => { let $a = risp!{$b}; };
    {(begin $($a:tt)*)} => {{$(risp!{$a});*}};
    {(lambda ($($a:tt)*) $b:tt)} => {(|$($a),*|risp!{$b})};
    
    {(if $cond:tt $t:tt)} => {if risp!{$cond} {risp!($t)}};
    {(if $cond:tt $t:tt $f:tt)} => {if risp!{$cond} {risp!($t)} else {risp!($f)}};
    
    {(loop $l:tt)} => {loop {risp!($l)}};
    {(while $cond:tt $l:tt)} => {while risp!{$cond} {risp!($l)}};

    {($fn:tt $($item:tt)*)} => {risp!($fn)($(risp!{$item}),*)};
}

macro_rules! risp {
    {($($item:tt)+)} => {risp_item_transfer!{($($item)+)}};

    {$unit:tt} => {risp_unit_checker!($unit)};
}