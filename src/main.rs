#![allow(non_snake_case)] //turns off warning for folder not being in snakecase

//UNCOMMENT A MODULE YOU WANT TO RUN. MAKE SURE IN main() THE EQUIVLANT RUN FUNCTION IS UNCOMMENTED

//mod print; //Similar to #include "filename" in c++
//mod variables;
//mod string;
//mod tuple_arrays;
//mod vectors;
//mod conditionals;
//mod loops;
//mod functions;
//mod structs;
fn main() {  //use cargo run to compile an run
    //print::run();  //:: is the scope modifer so you know what file to get a certain function from
    //variables::run();
    //string::run();
    //tuple_arrays::run_tuple();
    //tuple_arrays::run_array();
    //vectors::run();
    //conditionals::run();
    //loops::run();
    //functions::run();
    let a: i32 = i32::MAX; //10110 
    let b: i32 = 2;

    let result:i32;
    result = a << b;
    println!("{:b}", result);


}
