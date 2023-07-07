use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

fn from_into() {
    // let my_str = "hello";
    // let my_string = String::from(my_str);

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

fn try_from_into() {
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

fn convert_to_parse_from_string() {
    struct Circle {
        radius: i32,
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    let circle: Circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    println!("to_string() : {}", circle.radius.to_string());

    // Parse from
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed: i32 = "10".parse::<i32>().unwrap();

    let sum: i32 = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

fn main() {
    println!("########################################");
    println!("from_into");
    println!("########################################");
    from_into();
    println!("########################################");
    println!("try_from_into");
    println!("########################################");
    try_from_into();
    println!("########################################");
    println!("convert_to_parse_from_string");
    println!("########################################");
    convert_to_parse_from_string();
}
