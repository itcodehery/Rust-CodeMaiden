fn stackandheap(){
    //i32 is a stack-based type
    let stack1 = 32;
    let stack2 = stack1;
    println!(stack1);
    println!(stack2);
    
    //String is a heap-based type
    let heap1 = String::from("Hello");
    let heap2 = heap1.clone();
    println!("{heap1}");
    println!("{heap2}");
}