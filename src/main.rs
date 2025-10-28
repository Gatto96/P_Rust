// use std::io;

// fn sum(numbers: &[i32])-> i32{
//     let mut result=0;
//     for number in numbers{
//         result+= number;
//     }
//     result
// }

use core::num;


fn main() {

    //                          FUNCTIONS:3.0rs
    // Silence some warnings so they don't distract from the exercise.
    #![allow(unused_variables)]

        let number: f64 = 3.989; // don't change this line!

        // 1. Try running the code and looking at the error. We would like to use the variable name
        // `number` as an i32 (an integer), but it is already used as an f64 (a floating point number).
        //
        // - Uncomment the commented-out code below
        // - Complete the code to shadow the old `number` variable with a new `number` variable
        //   of the correct type.

        // fn convert_to_integer(number: f64)-> i32{ // uncomment this line and finish shadowing `number`
        //     return num.round() as i32 // uncomment this line
        // } // don't change this line!
        let number = convert_to_integer(number); // uncomment this line
        inspect_integer(number); // don't change this line!

        // 2. Uncomment and run the code below. Fix the scope problem so that the code compiles and runs
        // producing the answer 42.

        
            let answer = 42;
            println!("The answer is {}", answer);
        
        

        // 3. Create a function named `add` that adds two i32 values together and returns the result.
        // Then uncomment the code below. You should get the output "4 + 42 = 46"
        fn add(x: i32, y: i32) -> i32 {
             x + y
        }
        // Note: If you fixed the scope problem from #2 by moving the `println` up into the nested
        // scope, then you will have to change the code above again so that `answer` is in this scope.

        let sum=add(number, answer);// call your `add` function and pass it `number` and `answer` as arguments.
        println!("{} + {} = {}", number, answer, sum);

        // 4. You can declare a variable without initializing it, but the compiler must be able to
        // ensure that it will always be initialized before you can use it.
        //
        // Uncomment and run the code below to see the error. Fix the error by setting countdown to 0
        // in the `else` branch of the `if` expression. Run the code. You should see a countdown of 10.

        let countdown: i32; // declares countdown, but doesn't initialize it
        if answer < 100 {
            countdown = 10;
        } else {
            println!("The answer is clearly wrong.");
            countdown = 0;
            // set countdown to some value here
        }
        println!("The countdown begins at {}", countdown);
    

    fn inspect_integer(x: i32) {
        println!("The integer is {}", x);
    }

    // Challenge: A "tail expression" is when the last expression in a block does not end with a
    // semicolon, making it the value of the block.
    //
    // - Refactor the body of this function to be a "tail expression" instead of a return statement.
    // - Make the same change to the `add` function that you created
    // - Run the code and make sure you get the same output as you did before
    fn convert_to_integer(num: f64) -> i32 {
        // For more information on using `as` to cast between numeric types, see:
        // https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
        return num.round() as i32;
    }






    //              LEARNING VARIABLES_AND_MUTABILITY:2.0rs
    
        // let STARTING_MISSILES: i32=10;
        // let mut missiles: i32=STARTING_MISSILES;
        // let mut ready : i32=3;
        // missiles= STARTING_MISSILES - ready;
        // println!("Firing {} of my {} missiles... ", ready, STARTING_MISSILES);
        // println!("{} missiles left", missiles);
        
        // let mut READY_AMOUNT: i32=STARTING_MISSILES-ready;
        // println!("Firing {} of my {} missiles... ", ready, READY_AMOUNT);
        // println!("{} missiles left", READY_AMOUNT - ready);








    // println!("Hello, rusters!");

    // println!("Please enter a numbers to sum: ");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("failed to read numbers");

    // // Accept numbers separated by spaces and/or commas. Example: "1 2 3" or "1,2,3" or "1, 2, 3"
    // let numbers_vec: Vec<i32> = input
    //     .trim()
    //     .split(|c: char| c.is_whitespace() || c == ',')
    //     .filter(|s| !s.is_empty())
    //     .map(|s| s.trim().parse::<i32>())
    //     .filter_map(|r| match r {
    //         Ok(n) => Some(n),
    //         Err(e) => {
    //             eprintln!("Warning: couldn't parse '{}': {}", "<input>", e);
    //             None
    //         }
    //     })
    //     .collect();

    // if numbers_vec.is_empty() {
    //     println!("No valid numbers were provided.");
    // } else {
    //     let result = sum(&numbers_vec);
    //     println!("The sum is {}", result);
    // }



    // let mut i=0;
    // while i<5{
    //     println!("i={}",i);
    //     i+=1;
    // }

    // for i in 1..=10{
    //     println!("i={}",i);

    // }

    // for i in (1..=5).rev(){
    //     println!("{}",i);
    // }

    // let number =vec![1,2,3,4,5];
    // for n in number{
    //     println!("{}",n);
    // }

    // for i in 1..=10{
    //     if i % 2 ==0 {
    //         //skip even numbers
    //         continue; //mean skipping numbers
    //     }
    //     println!("i = {}", i);
    //     if i == 7 {
    //         //exit loop when is 7
    //         break; //stop;exit
    //     }
    // }

    //Match control flow

    // let name = "Hello";

    // //use of match to pattern match against varible "name"
    // match name {
    //     "Good bye" => println! ("Sorry to see you go"),
    //     "Hello" => println! ("Hi, Ruster, nice to meet you"),
    //     _ => println! ("I can't find a greeting, good bye"),
    // }

    // println!("Please enter a greeting: ");
    // let mut name= String::new();
    // io::stdin().read_line(&mut name).expect("Failed to read input");

    // //use of match expression to pattern against varible "name"
    // match name.trim(){
    //     "Good bye" => println! ("Sorry to see you go."),
    //     "Hello"=> println! ("Hi, Ruster, glad to meet you"),
    //     _ => println! ("I can't find a greeting, good bye"),
    // }



    
    


}
