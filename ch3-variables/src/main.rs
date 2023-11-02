fn main() {
    // variables and mutability
    const MY_CONTANT_VALUE: u32 = 2;
    let mut x = 5;
    let shadow = 2;
    let shadow = shadow + 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");
    println!("constand {MY_CONTANT_VALUE} and shadow {shadow}");

    // data types
    let _i8_value:i8 = 1;
    let _i16_value:i16 = 1;
    let _i32_value:i32 = 1;
    let _i64_value:i64 = 1;
    let _i128_value:i128 = 1;
    let _arch_value:isize = 1;

    let my_float:f32 = 1.0;
    let my_boolean:bool = true;
    let my_chart:char = 'a';

    // tuplas
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{my_float} {my_boolean} {my_chart}");
    let (x,y,z) = tup;
    let x_value = tup.0;
    println!("{x} {y} {z} {x_value}");

    // arrays
    let my_array = [1,2,3];
    let my_other_array: [i32;5] = [1,2,3,4,5];
    let my_initialized_array = [3;5];
    println!("{} {}",my_other_array[0],my_initialized_array[0]);

    my_other_function(my_array[1]);

    println!("{}",return_five());
}

fn my_other_function(number:i32){
    println!("{number}")
}
// when return a value we need to specify returned type after ->
fn return_five()->i32{
    5
}
