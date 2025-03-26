// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($single:expr) => {
        println!("You passed a single value: {}", $single);
    };
    ($first:expr, $second:expr) => {
        println!("You passed two values: {} and {}", $first, $second);
    };
    ($($rest:expr),+) => {
        let mut values = String::new();
        $(
            values.push_str(&format!("{}, ", $rest));
        )+
        if let Some(pos) = values.rfind(", ") {
            values.truncate(pos);
        }
        println!("You passed multiple values: {}", values);
    };
}
fn main() {
    my_macro!();
    my_macro!(42);
    my_macro!(10, 20);
    my_macro!(1, 2, 3, 4);
}

