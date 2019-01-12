macro_rules! bitwise {
    ($e:expr; and $r:expr) => (
        println!("{:?}", $e & $r);
    );

    ($e:expr; or $r:expr) => (
        println!("{:?}", $e | $r);
    )
}
