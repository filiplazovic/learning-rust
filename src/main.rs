#![allow(dead_code)]

// Macros
//-------------------------------------------------------------------------------------------------
use std::collections::HashMap;

#[macro_use]
mod simple_macro;
#[macro_use]
mod build_function;
#[macro_use]
mod print_expression;
#[macro_use]
mod bitwise_operations;
#[macro_use]
mod list_compr;
#[macro_use]
mod hash_map;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

fn macro_functions() {
    a_macro!(x => 10);

    build_fn!(some_function);
    some_function();

    print_ex!({
        let x = 10;
        let y = 20;
        x + y
    });

    bitwise!(5; and 3);
    bitwise!(5; or 2);

    let evens = list_compr!([0; 10], list_compr::even);
    let odds = list_compr!([0; 10], list_compr::odd);
    println!("{:?}", evens);
    println!("{:?}", odds);

    let new_map = new_hash_map!{
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };
    println!("{:?}", new_map);
}

// Debug
//-------------------------------------------------------------------------------------------------

mod debug;

fn debug_functions() {
    println!("{:?}", debug::Structure(7));
    println!("{:?}", debug::Deep(debug::Structure(8)));
    println!("{:#?}", debug::Person {name: "ninja", age: 8});

    let minmax = debug::MinMax(0, 14);
    println!("Display {}", minmax);
    println!("Debug {:?}", minmax);

    let point = debug::Point2D { x: 0.0, y: 1.0};
    println!("Display {}", point);
    println!("Debug {:?}", point);
}

// Literals
// ------------------------------------------------------------------------------------------------

fn literals() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 6 is {:04b}", 0b0001 << 0b0110);
    println!("0x80 >> 2 is {:04b}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);
}

// Custom types

mod custom_types;

fn custom_types() {
    let pressed = custom_types::WebEvent::KeyPress('x');
    let pasted = custom_types:: WebEvent::Paste("my text".to_owned());
    let click = custom_types::WebEvent::Click { x: 20, y: 80 };
    let load = custom_types::WebEvent::PageLoad;
    let unload = custom_types::WebEvent::PageUnload;

    custom_types::inspect(pressed);
    custom_types::inspect(pasted);
    custom_types::inspect(click);
    custom_types::inspect(load);
    custom_types::inspect(unload);

    println!("zero is {}", custom_types::Number::Zero as i32);
    println!("one is {}", custom_types::Number::One as i32);

    println!("roses are #{:06x}", custom_types::Color::Red as i32);
    println!("violets are #{:06x}", custom_types::Color::Blue as i32);
}

// LinkedList
// ------------------------------------------------------------------------------------------------

mod linked_list;

fn linked_list() {
    let mut list = linked_list::List::new();

    list = list.prepend(1);
    list = list.prepend(2);

    println!("{}", list.stringify());
}


// Mem size
// ------------------------------------------------------------------------------------------------

fn mem_size() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

// From, Into
// ------------------------------------------------------------------------------------------------

mod from_into;

fn from_into() {
    let num = from_into::Number::from(30);
    println!("My number is {:?}", num);

    let int = 30;
    let num: from_into::Number = int.into();
    println!("My number is {:?}", num);
}

// Generic types
// ------------------------------------------------------------------------------------------------

mod generic_types;
use crate::generic_types::{add, Detector, Detect, Collector, Lector};

fn where_clauses() {
    let res = add(5u32, 7u32);

    println!("{}", res);

    Detector::detect(2);
    Detector::detect("stf");
    Detector::detect2(2);
    Detector::detect2(2u32);
    Detector::detect_lector_collector(Collector { value: 3u32 });
    Detector::detect_lector_collector(Lector { name: "3u32".to_string() });
}

// JSON
// ------------------------------------------------------------------------------------------------

mod json_encode_decode;
use crate::json_encode_decode::do_stuff;

fn main() {
    do_stuff();
}
