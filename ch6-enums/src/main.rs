fn main() {
    // defining a enum: enumerating all possible options
    enum IpAddressKind  {
        V4,
        V6
    }

    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;


    // enums with type and value
    enum IpAddr {
        V4(String),
        V6(String)
    }
    println!("Hello, world!");
}
