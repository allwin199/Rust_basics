fn main() {
    let mut x: i32 = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

// In rust every variable is `immutable`
// To `mutate` a variable in rust
// mark it as `mut`

// x is of type `signed` integer
// i32 will be 32 bit integer

fn ununsedVariables() {
    let x = 1;
}

// If the above function is called, then compiler will throw error of unused variable
// to rectify it, we can use x => _x
// or

// [allow(unused_variables)]    => to allow unused variables
// fn ununsedVariables() {
//     let x = 1;
// }
