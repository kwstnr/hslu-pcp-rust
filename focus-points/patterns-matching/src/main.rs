enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Das Programm wird beendet.");
        }
        Message::Move { x, y } => {
            println!("Bewege zu Position ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Nachricht: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Farbe ändern zu RGB({}, {}, {})", r, g, b);
        }
    }
}

fn main() {
    let msgs = vec![
        Message::Write(String::from("Hallo Pattern Matching!")),
        Message::Move { x: 10, y: 20 },
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];

    for msg in msgs {
        process_message(msg);
    }
}
