// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($arg:expr) => {
        println!("Check out my macro with argument: {}", $arg);
    };
}

fn main() {
    my_macro!();
    my_macro!(42);
}
