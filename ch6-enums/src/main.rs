fn main() {
    // defining a enum: enumerating all possible options
    enum IpAddressKind {
        V4,
        V6,
    }

    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    // enums with type and value
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let my_ip: IpAddr = IpAddr::V4(String::from("1.1.1.1"));

    // enums with many types
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // implement functions into enums similar to structs
    impl Message {
        fn call(&self) {
            println!("call message");
        }
    }

    let my_message = Message::Write(String::from("hello there"));
    my_message.call();

    // define an enum to represent null values
    let my_null_value:Option<i8> = None;
}
