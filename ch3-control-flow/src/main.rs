fn main() {
    // conditionals
    let my_number = 32;
    if my_number>5 {
        println!("is greater than 5");
    }else if my_number==3{
        println!("is equals than 5");
    }
    else{
        println!("is less than 5");
    }

    let my_boolean = if my_number==3{true}else{false};
    println!("boolean result: {}",my_boolean);

    let mut counter = 0;

    // iterate with loop
    let my_value = loop {
        counter +=1;
        if counter==10 {
            break counter*10;
        }
    };

    println!("counter result: {my_value}");

    // iterate with while
    let mut i = 0;
    while i<3{
        println!("i: {i}");
        i+=1;
    }

    // iterate with for
    let my_array = [1,2,3,4,5];
    for element in my_array{
        println!("element: {element}");
    }

    // example of countdown
    for count in (1..4).rev(){
        println!("countdown: {count}");
    }
}
