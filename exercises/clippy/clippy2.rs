// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    // for x in option {
    //     res += x;
    // }
    // println!("{}", res);

    // [aziz]: Clippy produces `for_loops_over_fallibles` error for above code.
    //         Both lines are correct; `while let` and `if let`
    //         Explanations are here https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
    // while let Some(i) = option {res += i;}
    if let Some(i) = option { res += i; }
    println!("{}", res);
}
