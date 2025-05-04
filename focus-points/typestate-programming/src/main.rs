// Marker types for typestate
struct Disconnected;
struct Connected;

// Builder struct, generic over state
struct ConnectionBuilder<State> {
    state: State,
}

// Final usable struct
struct Connection;

impl ConnectionBuilder<Disconnected> {
    fn new() -> Self {
        ConnectionBuilder { state: Disconnected }
    }

    fn connect(self) -> ConnectionBuilder<Connected> {
        println!("Connecting...");
        ConnectionBuilder { state: Connected }
    }
}

impl ConnectionBuilder<Connected> {
    fn build(self) -> Connection {
        println!("Connection established.");
        Connection
    }
}

impl Connection {
    fn send(&self, msg: &str) {
        println!("Sending: {}", msg);
    }
}

fn main() {
    // Start disconnected
    let conn = ConnectionBuilder::new()
        .connect()  // only after this, you can build
        .build();

    // Now it's safe to use
    conn.send("Hello typestate!");
}


//You can’t call .build() unless .connect() has been called.
//You can’t call .send() unless .build() was called.
//The compiler prevents misuse by using different types at each step.