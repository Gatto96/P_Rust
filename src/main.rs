use std::io;

fn sum(numbers: &[i32])-> i32{
    let mut result=0;
    for number in numbers{
        result+= number;
    }
    result
}


fn main() {
    println!("Hello, rusters!");

    println!("Please enter a numbers to sum: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read numbers");

    // Accept numbers separated by spaces and/or commas. Example: "1 2 3" or "1,2,3" or "1, 2, 3"
    let numbers_vec: Vec<i32> = input
        .trim()
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<i32>())
        .filter_map(|r| match r {
            Ok(n) => Some(n),
            Err(e) => {
                eprintln!("Warning: couldn't parse '{}': {}", "<input>", e);
                None
            }
        })
        .collect();

    if numbers_vec.is_empty() {
        println!("No valid numbers were provided.");
    } else {
        let result = sum(&numbers_vec);
        println!("The sum is {}", result);
    }



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
