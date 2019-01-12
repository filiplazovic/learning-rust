macro_rules! new_hash_map {
    ($($key: expr => $val: expr),+,) => {{
        let mut map = HashMap::new();

        $(map.insert($key, $val);)*

        map
    }};
}
