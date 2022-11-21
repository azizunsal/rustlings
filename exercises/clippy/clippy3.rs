// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }
    if let Some(a) = my_option {};

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    // [aziz]: Clippy generates error for above line -->
    // --> error: this let-binding has unit value
    // --> help: omit the `let` binding: `vec![1, 2, 3, 4, 5].resize(0, 5);`

    vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_arr);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;

    // [aziz] Generated error was "error: this looks like you are trying to swap `value_a` and `value_b`" -->
    // -->  help: try: `std::mem::swap(&mut value_a, &mut value_b)`
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
