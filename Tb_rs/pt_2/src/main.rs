use std::io;
fn main() {
//NOTE You can't have add or multiply or divide or subtract different data types and bit size
//OVERFLOW- When we don't have enough bits to represent a value eg
//let x:u8=255; (u8 => 0-255)
//let y:u8=1;
//println!("{x+y}"); this would givean error be sum would be higher than the range for u8 values and there wouldn't be enough bits to represent the value
//NOTE: Arithmetic operations always give results in the same type. So if y:u8 = 10 and we do x/y, the answer would be 25 instead of 25.5 because u8 doesn't return a floating point value


//TYPE CASTING AND CONVERSION
//This can only be done e xplicitly
let _a=258_000.0f32; //this treats the value as an f32 type
//or
let _b=254.0_f32;
//or
let _c =32.0 as f32;

let i = 30u8;
let j = 5_i8;
let _k = i%(j as u8); // this converts j to a u8 so that the operation can be done

//NOTE!!! WHEN AN OVERFLOW OCCURS
let pa = (i32::MAX as i64)+1;// this results in the highest possible value of i32, converts it to i64 and adds 1
let pb = 10_i32;
let pc = pa as i32 / pb;
println!("{}", pc); // -214748364 (you get this value because an overflow has occurred. You are outside the range of i32). We use TOOLS COMPLEMENT WRAPPING to fix this. We'll look at that later.

//CONVERTING STRING TO INTEGER
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read input");
let t_input:i32=input.trim().parse().unwrap();
//.unwrap() => Give me the result of the computation, and if there was an error, panic and stop the program
println!("{t_input}");
//NOTE !! This only works for strings that can be parsed into integers(e.g 55, -7). If I put in helllo or 9.7, it gives an error
}
