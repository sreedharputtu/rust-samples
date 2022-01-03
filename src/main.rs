#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;

fn main() {
    /**
    Signed and unsigned
    1 . u - un signed -> for example u8 size is 0...256
    2 . i - signed -> for example 18 size is -128...127
    unsigned types
    u8 , u16 , u32 etc
    signed types
    i8 , i16 , i32 etc
     **/

    let s: u8 = 255; // this creates an immutable variable
    println!("{}", s);
    //if you re assign the value to s ,compilation will fail
    //s = 122;

    let p: i8 = 127;
    println!("{}", p);

    /**
    mutable variable
    **/

    let mut q: u8 = 256;
    q = 78;
    println!("{}", q);

    /**
    if type is not mentioned explicitly , then rust compiler defaults it automatically
    in below case c is i32 type
    **/
    let c = -1;
    println!("{},{}", c, mem::size_of_val(&c));

    /**
    usize and isize are data types which  depends on OS
    (32 bit or 64 bit based on OS)
    **/
    let d: usize = 1;
    let size_of_d = mem::size_of_val(&d);
    println!("{} bit", size_of_d * 8);

    /**
    floating points , f64 is default type for floating point variable
    f32 , f64 etc
    **/
    let g = 6.7;
    println!("{}", mem::size_of_val(&g));
    /**
    bool
    **/
    let x = true;
    println!("{},{}", x, mem::size_of_val(&x));
}
