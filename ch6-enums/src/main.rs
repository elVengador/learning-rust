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
    let my_null_value: Option<i8> = None;

    // match control flow

    #[derive(Debug)]
    enum UsStates {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsStates),
    }

    fn value_in_cents(coin: Coin) -> i8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Us State: {:?}", state);
                25
            }
        }
    }

    println!("cents of Dime: {}", value_in_cents(Coin::Dime));
    println!("cents of Dime: {}", value_in_cents(Coin::Quarter(UsStates::Alabama)));

    // match option<T>
    fn plus_one (x:Option<i8>)->Option<i8>{
        match x {// into match should be defined every possible case
            None=>None,
            Some(i)=>Some(i+1)
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // match other values
    let dice_roll = 9;
    match dice_roll {
        7=>println!("the value is 7"),
        3=>println!("the value is 3"),
        (other)=>println!("you have other value: {}",other)
    }

    // match other value without use the value
    let value  = 2;
    match value {
        3=>println!("the value is 3"),
        _=>()// with this empty parenthesis you don't make nothing
    }
}
