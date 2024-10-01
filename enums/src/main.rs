/*
Enum and struct are using in different situation. struct can gether the related data, like Rec gether the
width and height, but enum gether the possible thing, for example, rectangle, square, circle..., and enum
put these possible things into the same type.
*/

// This enum is okay, but if want to give some value for the enum's variant, it can't
// enum IpAddrKind {
//     V4,
//     V6,
// }
// This enum's variant can get value by a function call(like constructor)"IpAddr::V4()" or V6.
/*
Another advantage of enum rather than struct is each variant can have different structure or type, but
they still in same type "IpAddr".
*/
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8 ,u8),
    V6(String),
}

impl IpAddr {
    fn print(&self) {
        match self {
            IpAddr::V4(a, b, c, d) => {
                println!("The IP is: {}.{}.{}.{}", a, b, c, d);
            }
            IpAddr::V6(addr) => {
                println!("The IP is: {}", addr);
            }
        }
    }
}

fn main() {
    // four and six are both IpAddrKind type
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));
    four.print();
    six.print();
}