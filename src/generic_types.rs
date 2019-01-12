pub struct Detector {}

#[derive(Debug)]
pub struct Collector {
    pub value: u32
}

#[derive(Debug)]
pub struct Lector {
    pub name: String
}

trait CollectorDetector {
    type Collector;
    type Detector;
}

pub trait Detect<T, B, C> {
    fn detect(input: T);
    fn detect2(input: B);
    fn detect_lector_collector(input: C);
}

impl Detect<i32, u32, Collector> for Detector {
    fn detect(input: i32) {
        println!("{}", input);
    }
    fn detect2(input: u32) {
        println!("{}", input);
    }
    fn detect_lector_collector(input: Collector) {
        println!("{:?}", input);
    }
}

impl Detect<&str, i32, Lector> for Detector {
    fn detect(input: &str) {
        println!("{}", input);
    }
    fn detect2(input: i32) {
        println!("{}", input);
    }
    fn detect_lector_collector(input: Lector) {
        println!("{:?}", input);
    }
}

pub fn add<T: Into<u32>>(a: T, b: T) -> u32 {
    a.into() + b.into()
}
