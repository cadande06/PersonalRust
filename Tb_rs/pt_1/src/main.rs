use std::io;
fn main() {
    //four spaces indentation
    println!("Hello, world!");
    let  mut /*shows that x is changeable */ x= 4; //explicit variable type assignment !!!but you can't change the data type or it will give an error
    println!("x is : {}",x);
    x = 5;
    println!("x is : {}",x);
    
    //to redefine the x variable without using mut keyword abd you can change the data type
    let y = 5;
    println! ("{}",y);
    let y = "hi";
    println!("{}",y);

    //NAME SHADOWING:using the same variable name but for different scopes
    /*a block of code or scope*/{
        let y = 50;
        println!("{}",y);//prints 50
    }
    
    //NOTE:You can use variables from exterior scopes in interior scopes
    {
        let x = x+15;
        println!("x is : {}",x);//prints x=5+15=>20
    }

    //CONSTANTS
    /*
    -Their values and data types cannot change throughout the code
    -They must be in uppercases and be separated by _
    -Their types must be specified and values must always be assigned to them 
    */
    const SECONDS_IN_MINS:u32 = 60;
    println!("{}",SECONDS_IN_MINS);

    /*
    DATA TYPES
    -Primitive Datat types: they are basic
    TYPES
    1.Scalar data types: represet single values => floating point, characters, boolean, integers, unit
    UNIT-empty parentheses ()
    INTEGERS-positive or negative whole numbers
    TYPES-signed(represent both positve and negative integers i32, i64) & unsigned( represent only psoitive integers u32, u8)
    NOTE:THE number represents the amount of space in bits the variable can take up and for example, u8 represents 0 to 2^8-1 unique values because its unsigned and doesn't take negative values.
    The default bits for varibles in rust is 32.i8 repersents from -2^7 to 2^7 - 1 So rust implicitly assigns 32 bits to variables

    FLOATING POINT(DECIMAL NUMBERS EG 5.0, 3.5)
    We have only two types=> f32(single precision) & f64(double precision)

    BOOLEAN
    true or false value or 1 or 0 (true/false use lowercases only) .ln 64 .

    CHARACTER TYPE(char)
    stores single characters in single quotation marks. It could be a number or letter or symbols. ln 65 .
     
    2.Compound data types: represent several values.Formed from scalar types and other composite tyes => tuples, array
    TUPLES-Sequence of immutable elements.The type depend on the elements inside. You can use mut to make them mutable.They us eround brackets.To reference an element, you use tuple_name.index . ln 66 .
    
    ARRAYS-Surroounded by []. Their elements must have the same data types. You can't add extra elements into the array.It's defined by the data type and nmber of elements in the array. ln 72 .
    */

    let _res:bool=false;
    let _ca:char='h';
    let mut  tup:(i32, bool, char)=(4, true,'s');
    println!("{}",tup.0); 
    //println!("{}",tup); this won't work because it can't be printed in this format
    tup= (5, false, '4');
    println!("{}",tup.0); 

    let mut arr:[i32;5]=[1,2,3,4,5];
    arr[4]=10;
    println!("{}",arr[4]);

    //NOTE
    let ru:u8=4;
    let bi=ru;
    println!("{}",bi);//this prints 4 and the type will be u8. you can't change the type

    /*
    COLLECTING USER INPUTS
    Prelude => list of things that Rust automatically imports into every Rust program. This is what lets us use keywords or functionalities like fn, println and so on. 
    Crates => fundamental compilation unit of Rust code. It has two types -- binary and library
    Library => A collection of reusable, precompiled programs, scripts, or routines that can be called by the programmer while writing a code. In libraries, we have modules.
    Module =>  a collection of items: functions, structs, traits, impl blocks, and even other modules.
    e.g
    use std::io;
    std - standard library
    io - input output module
    */
    
    let mut input = String::new();//this creates a new mutable variable where we'll assign our input. Rust has two tyes of strings -- String(when you need to own a string an change its contents) and str(for read only strings)
    //:: => path separator operator
    //new() => function
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);//will print out whatever the user types
    /*
    stdin--standard input function from the io module.  
    This code(ln 95) allows us to directly modify the data that's stored in the 'input' variable.Also,, we use it to collect input. We are creating a mutable reference to the input variable. Originally, putting a variable as parameter of a function (e.g read_line(input)) only modfies a copy of the variable and not the actual variable. 
    So, we use a reference called &, which is originally immutable, so we use a mut keyword with it.
    read_line() => function used to read data, one line at a time from an input stream or file
    .expect() => this is used incase the read_line function doesn't work.
    SOME METHODS IN RUST
    .trim() => removes whitespaces from around the string
    .parse() => converts a string into an integer
    .abs() => returns absolute value
    format! => write formatted text to String
    print! => same as format! but the text is printed to the console (io::stdout).
    println! => same as print! but a newline is appended.
    */
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");//you can also print this way
    let why = "hi";
    println!("{why}");//you can also print this way
    //Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    //usize => an unsigned integer type with the same number of bits as the platform’s pointer type. It can represent every memory address in the process.
    let width:usize=20;

    /*
    fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
    fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.
    So fmt::Debug definitely makes this printable but sacrifices some elegance. Rust also provides "pretty printing" with {:#?}
     */

    
    
}
