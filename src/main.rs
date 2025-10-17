use std::io;


fn main() {
    println!("Hello, rusters!");



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

    println!("Please enter a greeting: ");
    let mut name= String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    //use of match expression to pattern against varible "name"
    match name.trim(){
        "Good bye" => println! ("Sorry to see you go."),
        "Hello"=> println! ("Hi, Ruster, glad to meet you"),
        _ => println! ("I can't find a greeting, good bye"),
    }

    
    


}
