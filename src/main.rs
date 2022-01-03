#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;
use std::mem::transmute;

fn main() {

    // u - un signed -> 0...256
    // i - signed -> -128...127
    //u8 , u16 , u32 ... i8,i16,i32 etc
   let s :u8=255; // this creates an immutable variable
    println!("{}",s);
    //if you re assign the value to s , compiler will fail
    //s = 122;s

    let p :i8 = 127;
    println!("{}",p);

    let c = -1;
    println!("{},{}",c,mem::size_of_val(&c));
    //if you dont mention type , IDE/compiler defaulting value to inteeger 32 bit signed

    let d:usize = 1;
    let size_of_d = mem::size_of_val(&d);
    println!("{} bit",size_of_d*8);

    let g = 6.7;
    println!("{}",mem::size_of_val(&g));

   let x = (1 |  0);
 println!("{},{}",x , mem::size_of_val(&x));

 let mut xp:u8 = 22;
  xp +=2;
 println!("{}",xp);

}
