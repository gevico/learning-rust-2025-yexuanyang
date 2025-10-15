// clippy3.rs
//
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;

    let my_arr = &[-1, -2, -3 - 4, -5, -6];
    println!("My array! Here it is: {my_arr:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    let mut tmp = 0;
    // Let's swap these two!
    tmp = value_a;
    value_a = value_b;
    value_b = tmp;
    println!("value a: {value_a}; value b: {value_b}");
}
