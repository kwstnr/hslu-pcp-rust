# Typestate-Programmierung in Rust

Dieses Projekt zeigt das **Typestate-Muster** in Rust anhand eines einfachen Beispiels: Ein `ConnectionBuilder`, der nur in einer bestimmten Reihenfolge verwendet werden kann.

---

## Was ist Typestate?

Typestate bedeutet, dass der **Zustand eines Objekts im Typ** codiert ist. Dadurch stellt der Compiler sicher, dass nur erlaubte Methoden in einem bestimmten Zustand aufgerufen werden können.

Beispiel:

- Du musst `.connect()` aufrufen, bevor `.build()` verfügbar ist.
- Du kannst `.send()` nur nach erfolgreichem `.build()` aufrufen.
- Fehlerhafte Aufrufe führen zu einem Compilerfehler, nicht zu einem Laufzeitfehler.

---

## Vorteile

- Compiler-Sicherheit statt Laufzeitfehler
- Zwang zur korrekten Reihenfolge (z. B. konfigurieren → verbinden → senden)
- Ungültige Zustände sind gar nicht erst darstellbar

---

## Vergleich zu Enums

|                | Typestate                   | Enum-basierte Zustände         |
|----------------|-----------------------------|--------------------------------|
| Compiler-Schutz| ✅ Ja                        | ❌ Nein                         |
| Laufzeitfehler | ❌ Unmöglich                 | ⚠️ Möglich, wenn falsch genutzt |
| Lesbarkeit     | ⚠️ Etwas mehr Code           | ✅ Einfach                      |

---

## Beispielnutzung

```rust
let conn = ConnectionBuilder::new()
    .connect()
    .build();

conn.send("Hallo Typestate!");
```

Fehlende Schritte wie `.connect()` führen zu einem Kompilierfehler, nicht zu einem Absturz zur Laufzeit.

---

## 🔗 Mehr Infos

- [Rust Embedded Book: Typestate Programming](https://docs.rust-embedded.org/book/static-guarantees/typestate-programming.html)

---

## 🚀 Projekt ausführen

```bash
cargo run
```

Ausgabe:

```bash
Connecting...
Connection established.
Sending: Hallo Typestate!
```
