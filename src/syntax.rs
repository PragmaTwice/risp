trace_macros!(false);

macro_rules! risp_unit_checker {
    ($unit:literal) => ($unit);
    ($unit:ident) => ($unit);
    (_) => (_);
}

macro_rules! risp_item_transfer {    
    {(define $p:tt $v:tt)} => { let risp!($p) = risp!{$v}; };

    {(tuple $($v:tt)+)} => { ($(risp!($v),)+) };

    {(mut $p:tt)} => { mut $p };

    {(begin $($item:tt)+)} => {{ $(risp!{$item});+ }};
    {(lambda ($($para:tt)*) $item:tt)} => {(|$($para),*|risp!{$item})};
    
    {(if $cond:tt $t:tt)} => {if risp!{$cond} {risp!($t)}};
    {(if $cond:tt $t:tt $f:tt)} => {if risp!{$cond} {risp!($t)} else {risp!($f)}};
    {(match $v:tt $((=> $p:tt $q:tt))*)} => {match risp!{$v} {$(risp!{$p} => risp!{$q}),*}};
    
    {(loop $l:tt)} => {loop {risp!($l)}};
    {(while $cond:tt $l:tt)} => {while risp!{$cond} {risp!($l)}};
    { break } => { break };
    { continue } => { continue };
    
    {(+ $v1:tt $($vn:tt)+)} => {risp!($v1) $(+ risp!{$vn})+};
    {(- $v1:tt)} => {- risp!($v1)};
    {(- $v1:tt $v2:tt)} => {risp!($v1) - risp!{$v2}};
    {(* $v1:tt $($vn:tt)+)} => {risp!($v1) $(* risp!{$vn})+};
    {(/ $v1:tt $v2:tt)} => {risp!($v1) / risp!{$v2}};
    {(% $v1:tt $v2:tt)} => {risp!($v1) % risp!{$v2}};

    {(& $v1:tt $($vn:tt)+)} => {risp!($v1) $(& risp!{$vn})+};
    {(| $v1:tt $($vn:tt)+)} => {risp!($v1) $(| risp!{$vn})+};
    {(^ $v1:tt $($vn:tt)+)} => {risp!($v1) $(^ risp!{$vn})+};
    {(<< $v1:tt $v2:tt)} => {risp!($v1) << risp!{$v2}};
    {(>> $v1:tt $v2:tt)} => {risp!($v1) >> risp!{$v2}};
    
    {(= $v1:tt $v2:tt)} => {risp!($v1) = risp!{$v2}};
    {(+= $v1:tt $v2:tt)} => {risp!($v1) += risp!{$v2}};
    {(-= $v1:tt $v2:tt)} => {risp!($v1) -= risp!{$v2}};
    {(*= $v1:tt $v2:tt)} => {risp!($v1) *= risp!{$v2}};
    {(/= $v1:tt $v2:tt)} => {risp!($v1) /= risp!{$v2}};
    {(%= $v1:tt $v2:tt)} => {risp!($v1) %= risp!{$v2}};
    {(&= $v1:tt $v2:tt)} => {risp!($v1) &= risp!{$v2}};
    {(|= $v1:tt $v2:tt)} => {risp!($v1) |= risp!{$v2}};
    {(^= $v1:tt $v2:tt)} => {risp!($v1) ^= risp!{$v2}};
    {(<<= $v1:tt $v2:tt)} => {risp!($v1) <<= risp!{$v2}};
    {(>>= $v1:tt $v2:tt)} => {risp!($v1) >>= risp!{$v2}};

    {(== $v1:tt $v2:tt)} => {risp!($v1) == risp!{$v2}};
    {(!= $v1:tt $v2:tt)} => {risp!($v1) != risp!{$v2}};
    {(< $v1:tt $v2:tt)} => {risp!($v1) < risp!{$v2}};
    {(<= $v1:tt $v2:tt)} => {risp!($v1) <= risp!{$v2}};
    {(> $v1:tt $v2:tt)} => {risp!($v1) > risp!{$v2}};
    {(>= $v1:tt $v2:tt)} => {risp!($v1) >= risp!{$v2}};

    {(! $v1:tt)} => {! risp!($v1)};
    {(&& $v1:tt $($vn:tt)+)} => {risp!($v1) $(&& risp!{$vn})+};
    {(|| $v1:tt $($vn:tt)+)} => {risp!($v1) $(|| risp!{$vn})+};

    {(as $v1:tt $v2:tt)} => {risp!($v1) as risp!{$v2}};

    {($fn:tt $($item:tt)*)} => {risp!($fn)($(risp!{$item}),*)};
}

macro_rules! risp {
    {($($item:tt)+)} => {risp_item_transfer!{($($item)+)}};

    {$unit:tt} => {risp_unit_checker!($unit)};
}