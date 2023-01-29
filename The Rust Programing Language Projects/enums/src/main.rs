
mod main2;
mod main3;
mod main4;
mod main5;

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    main2::main2();
    main3::main3();
    main4::main4();
    //main5::main5();
}

fn route(ip_kind: IpAddrKind) {}