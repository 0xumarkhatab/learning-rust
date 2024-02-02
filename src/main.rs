use std::any::type_name;

fn variables() {
    let mut x: i32 = 10;
    println!("value of x {x}");
    x = 12;
    println!("value of x {x}");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, _) = tup;
    print!("{a} {b}");
    // let tup1 = ();
    // print!({ type_name(tup1) + "" });
}
fn main() {
    variables()
}
