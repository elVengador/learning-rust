fn main() {
    // defining a struct
    struct User  { 
        active:bool,
        username:String,
        email:String,
        sign_in_count:u64,
    }
    let my_user = User{
        active:true,
        username:String::from("lalo"),
        email:String::from("some@gmail.com"),
        sign_in_count:2
    };

    println!("Hello, {}!",my_user.username);

    // defining a mutable struct
    let mut user2 = User{
        username:String::from("Marco"),
        ..my_user
    };
    user2.username = String::from("Marco Polo");
    println!("Hello, {}!",user2.username);

    // defining a tuple struct
    struct Color (i32,i32,i32);
    let my_color = Color(12,23,22);
    println!("my color: {}, {} ,{}",my_color.0,my_color.1,my_color.2);

    // defining a unit-like struct
    struct AlwaysEquals;

    // using structs in a simple project
    #[derive(Debug)]
    struct Rectangle  {
        width:i32,
        heigh:i32
    }

    fn calculate_area(rectangle:Rectangle)->i32{
        dbg!(&rectangle);// this macro allows to print a struct
        rectangle.heigh*rectangle.width
    }

    let my_rectangle = Rectangle{
        heigh:10,
        width:20
    };

    println!("result: {}",calculate_area(my_rectangle));

    // define methods

    #[derive(Debug)]
    struct Circle {
        radio:i8,
        color:String
    }

    impl Circle { // create associated function into impl
        fn build_circle(radio:i8)->Self{// this function return a new instances, and is called with ::
            Self { radio, color: String::from("red") }
        }
        fn get_area(&self)->f32{
            f32::from(self.radio)*3.14
        }
    }

    let my_circle = Circle::build_circle(2);
    dbg!(&my_circle);
    println!("color of circle: {}",my_circle.color);
}
