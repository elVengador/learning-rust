fn main() {
    let s1 = String::from("Hi"); // define s1 in this scope
    let (s,l) = calculate_size(s1);          // move s1 ownership to calculate_size scope
                                             // also is moved s ownership into s
    // we cant use s1 variable
    println!("{s} and {l}");
}

fn calculate_size(s:String) -> (String,usize){
    let length = s.len();   // we can use s variable because is in this scope and the ownership is here
    (s,length)
}// the ownership removes the s variable from heap memory
