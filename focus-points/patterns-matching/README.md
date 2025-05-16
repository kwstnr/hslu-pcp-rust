# Pattern Matching in Rust

Dieses Projekt zeigt, wie **Pattern Matching** in Rust funktioniert – eines der mächtigsten Werkzeuge der Sprache. Mit `match` kannst du Enums, Structs, Tupel und mehr direkt **destrukturieren und gezielt verarbeiten**.

## Was ist Pattern Matching?

Pattern Matching bedeutet, dass du **Werte strukturieren und analysieren** kannst – direkt beim Vergleich.

Beispiel:  
Ein `Message`-Enum mit verschiedenen Varianten wird je nach Inhalt unterschiedlich behandelt:

```rust
match msg {
    Message::Quit => { ... }
    Message::Move { x, y } => { ... }
    Message::Write(text) => { ... }
    Message::ChangeColor(r, g, b) => { ... }
}
```

## Vorteile

- **Sicher:** Der Compiler stellt sicher, dass alle Varianten behandelt werden.
- **Ausdrucksstark:** Kein verschachteltes `if`, sondern klare Struktur.
- **Mächtig:** Funktioniert mit Enums, Structs, Tupeln, Arrays usw.

## Beispielausgabe

```bash
Nachricht: Hallo Pattern Matching!
Bewege zu (10, 20)
Farbe: RGB(255, 0, 0)
Programm wird beendet.
```

## Mehr Infos

- [The Rust Book: Chapter 19 – Patterns and Matching](https://doc.rust-lang.org/book/ch19-00-patterns.html)
