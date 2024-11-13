fn main() {
    // CONDITIONS 
    /*
    They always evaluate to boolean - true/false
     SIGNS - >,<,==,<=,>=,!= 
     NOTE when you compare values of different data types, it gives errors
     */
    let cond = 2<3;
    
    /*
    COMPOUND CONDITIONS 
    -mutliple conditions chained together using logical operators => AND &&, OR ||,  NOT !
    */

    let cond2 = true && cond;//checks if both the left and right are true 
    println!("{cond2}");//true
    let cond3 = !(true && cond);//flips whatever boolean is here
    println!("{cond3}");//false 
    //ORDER when applying the operators ! && ||, but whatever in parenthesis is applied first

    //CONTROL FLOW 
    //IF STATEMENTS
     let food = "cake";
     if food == "cookie"{ 
         println!("I like {food} too!");
     }else if food == "cake"{
        println!("Tu aimes cake.J'aime cookies. Avons a la boulangerie ce soir!");
     } else{
        println!("Oh, that's too bad");
     }

     //MATCH STATEMENTS
     let number = 13;
    

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
       };
       println!("{} -> {}", boolean, binary); 

       
     test_one();//this is how we call functions
     add_no(5, 7);
     let result=div_no(225,15);
     println!("{result}");
     let product=mult_no(3.0, 12.0);
     println!("{product}");
     //STATEMENTS-a variable declaration(e.g let x = 20;).It doesn't return a value or evaluate to something. You can't write let y = x = 20; in rust. In rust, we have statements not expressions.
    //Functions are also only statements. you can't write let x=add_no(x,y){println!("Hi")}; 
    //EXPRESSION - anythign you write in rust that evaluates to something or returns a value. A function call is an expression. A macro is an expression.Numbers are expressions. 2<3, because it evaluates to true/false. Blocks {} are also expressions
let no={
    let x=3;
    x+1
}; // this evauates to 4 and assigns it to no. The let no is a statement, but its content is an expression. We are assigning its content to no. It returned a value, 4.Don't put a semicolon to return the value
println!("{no}");
}
//FUNCTIONS
//Snake case is used for function names. You can define your function anywhere-above or below. You can have parameters and always specify their types. 
fn test_one(){
    println!("Je veux travailler a Londres bientot!"); 
}//ln 31
fn add_no(x:i32, y:i32){
    println!("The sum is {}", x+y); 
}//ln 32

//HOW TO RETURN VALUES FROM FUNCTIONS
fn div_no(x:i32, y:i32) -> i32{
    x/y // this is an expression which will be returned when the function is called and you have to specify the data type of the return value
}//line 33
//OR
fn mult_no(x:f32, y:f32) -> f32{
    let result = x*y;
    if result > 10.0{
        return result-10.0;
    }
    result
}//ln 35

