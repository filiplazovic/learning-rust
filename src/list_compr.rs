macro_rules! list_compr {
    ([$start: expr; $end: expr], $cond: expr) => {{
        let mut vec = Vec::new();

        for num in $start..$end + 1 {
            if $cond(num) {
                vec.push(num);
            }
        }
        vec
    }};
}

pub fn even(x: u32) -> bool {
    return x % 2 == 0;
}

pub fn odd(x: u32) -> bool {
    return x % 2 != 0;
}
