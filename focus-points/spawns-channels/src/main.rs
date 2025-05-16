use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Erstelle einen Kanal (Sender, Empfänger)
    let (tx, rx) = mpsc::channel();

    // Starte einen neuen Thread und sende eine Nachricht
    thread::spawn(move || {
        let message = "Hallo von einem anderen Thread!";
        println!("Child-Thread: sende Nachricht…");
        tx.send(message).unwrap(); // sendet über den Kanal
        thread::sleep(Duration::from_secs(1));
        println!("Child-Thread: fertig.");
    });

    // Haupt-Thread wartet auf Nachricht
    println!("Haupt-Thread: warte auf Nachricht…");
    let received = rx.recv().unwrap();
    println!("Haupt-Thread: erhalten -> {}", received);
    
    thread::sleep(Duration::from_secs(2)); //<- Warten auf den Child-Thread
}
