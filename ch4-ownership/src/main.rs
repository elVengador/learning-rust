fn main() {
    //-----------------
    // ownership:
    let s1 = String::from("Hi"); // define s1 in this scope
    let (s, l) = calculate_size(s1); // move s1 ownership to calculate_size scope
                                     // also is moved s ownership into s
                                     // we cant use s1 variable
    println!("{s} and {l}");

    //-----------------
    // references:
    // we can borrow a variable so is not required to return the variable in the function
    // also we can transform a reference into a mutable reference &mut my_var
    let s2 = String::from("bye"); // define s1 in this scope
    let ll2 = calculate_size_with_reference(&s2); // move s1 ownership to calculate_size scope
                                                  // also is moved s ownership into s
                                                  // we cant use s2 variable because was only borrowed
    println!("{s2} and {ll2}");

    //-----------------
    // slices:
    // references continue elements from a collection
    // it contains the reference to the first element and the len
    // string Slices
    let mut s = String::from("hello world");
    let string_slices = &s[0..5];// slice the string
    let (size,str) = get_len(string_slices);   // send a reference from 0 to 4
                                                             // get the slice
    println!("{} {}", size,str);    // we can use str because is valid in this moment
    s.clear();    // after this the str is not valid and we cant use it

    // other slices
    let a = [1,2,3];
    let number_slice = &a[0..1];
    assert_eq!(number_slice, &[1]);
}

fn calculate_size(s: String) -> (String, usize) {
    let length = s.len(); // we can use s variable because is in this scope and the ownership is here
    (s, length)
} // the ownership removes the s variable from heap memory

fn calculate_size_with_reference(s: &String) -> usize {
    // s is borrowed so this is immutable
    let length = s.len();
    length
} // the ownership doesn't removes the s variable from heap memory

fn get_len(s: &str) -> (usize,&str) {
    (s.len(),&s[..])
}
