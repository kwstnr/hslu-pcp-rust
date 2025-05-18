# Channels

Rust bietet asynchrone Kanäle für die Kommunikation zwischen Threads. Kanäle ermöglichen einen unidirektionalen Informationsfluss zwischen zwei Endpunkten: dem Absender und dem Empfänger.

## Erklärung

### 1. `mpsc::channel()`

- Erstellt einen **Sender (tx)** und **Empfänger (rx)**.
- `mpsc` steht für "**Multiple Producer, Single Consumer**".

### 2. `thread::spawn(...)`

- Startet einen neuen **Child-Thread**.
- Der `Sender` wird via `move` hineingeschoben und genutzt.

### 3. `tx.send(...)`

- Sendet eine Nachricht **vom Child-Thread** zum Haupt-Thread.

### 4. `rx.recv()`

- Der Haupt-Thread **wartet**, bis die Nachricht eintrifft.

## Ausgabe

```bash
Haupt-Thread: warte auf Nachricht…
Child-Thread: sende Nachricht…
Haupt-Thread: erhalten -> Hallo von einem anderen Thread!
Child-Thread: fertig.
```

## Was verhindert der Compiler?

- `tx` kann nicht ohne `move` in den neuen Thread kopiert werden → **Sicherheit durch Ownership**
- `recv()` ist blockierend, sorgt für **synchrones Warten**

## Mehr Infos

- [Rust By Example: Channels](https://doc.rust-lang.org/rust-by-example/std_misc/channels.html)