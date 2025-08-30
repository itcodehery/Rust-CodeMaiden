#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize { width: i32, height: i32 },
    Move { point: Point },
    Echo { str: String },
    ChangeColor { x: i32, y: i32, z: i32 },
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move {
            point: Point { x: 10, y: 15 },
        },
        Message::Echo {
            str: String::from("hello world"),
        },
        Message::ChangeColor {
            x: 200,
            y: 255,
            z: 200,
        },
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
