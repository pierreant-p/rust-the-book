enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// struct IPV4Addr {
//     // snip
// }

// struct IPV6Addr {
//     // snip
// }

// enum IPAddr {
//     V4(IPV4Addr),
//     V6(IPV6Addr),
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // do something
    }
}

fn main() {
    let _home = IPAddr::V4(127, 0, 0, 1);
    let _loopback = IPAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let null_number: Option<i32> = None;

    // This doesn't work because we can't add a value and an Option
    // We need to get the value of the option first
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
}

fn route(ip_addr: IPAddr) {
    // do something useful
}
