macro_rules! say_hello {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

say_hello!(foo);
fn main() {
    foo();
}
