fn main() {
    const MY_CONTANT_VALUE: u32 = 2;
    let mut x = 5;
    let shadow = 2;
    let shadow = shadow + 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");
    println!("constand {MY_CONTANT_VALUE} and shadow {shadow}");
}
