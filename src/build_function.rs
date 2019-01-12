macro_rules! build_fn {
    ($func_name:ident) => (
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    );
}
