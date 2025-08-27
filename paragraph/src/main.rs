fn main() {
    // variable
    let mut str = String::new();

    // user input
    println!("Enter a paragraph: ");
    std::io::stdin().read_line(&mut str).unwrap();

    //processing string
    let str = str.trim();
    let str: Vec<&str> = str.split(" ").collect();
    // println!("{:?}", str);

    let mut out: Vec<&str> = Vec::new();
    // conditional assignment
    let _ = str.iter().for_each(|x| {
        if x.len() == 5 {
            out.push(x);
        }
    });
    // print output to the screen
    println!("{:?}", out);
}
