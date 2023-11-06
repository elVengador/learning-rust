fn main() {
    // ownership:
    let s1 = String::from("Hi"); // define s1 in this scope
    let (s,l) = calculate_size(s1);          // move s1 ownership to calculate_size scope
                                             // also is moved s ownership into s
    // we cant use s1 variable
    println!("{s} and {l}");

    // references:
    // we can borrow a variable so is not required to return the variable in the function
    let s2 = String::from("bye"); // define s1 in this scope
    let ll2 = calculate_size_with_reference(&s2);          // move s1 ownership to calculate_size scope
                                             // also is moved s ownership into s
    // we cant use s2 variable because was only borrowed
    println!("{s2} and {ll2}");
}

fn calculate_size(s:String) -> (String,usize){
    let length = s.len();   // we can use s variable because is in this scope and the ownership is here
    (s,length)
}// the ownership removes the s variable from heap memory

fn calculate_size_with_reference(s:&String) -> usize{ // s is borrowed so this is immutable
    let length = s.len();
    length
}// the ownership doesn't removes the s variable from heap memory
