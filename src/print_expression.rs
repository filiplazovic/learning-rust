macro_rules! print_ex {
    ($e:expr) => (
        println!("{:?} = {:?}", stringify!($e), $e);
    );
}
