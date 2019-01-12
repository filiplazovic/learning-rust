macro_rules! a_macro {
    (x => $e:expr) => (
        println!("{}", $e);
    );
}
