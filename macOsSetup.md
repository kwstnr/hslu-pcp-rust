# Rust Setup auf macOS

## Voraussetzung: C Compiler (Xcode Command Line Tools)

Rust benötigt einen **C-Compiler** für bestimmte Abhängigkeiten. Dieser ist normalerweise bereits vorhanden. Falls nicht, kannst du ihn so installieren:

1. Öffne das Terminal.
2. Führe folgenden Befehl aus:
   ```sh
   xcode-select --install
   ```

---

## Rust installieren
Öffne ein Terminal und führe folgenden Befehl aus:
```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

---

## Verifikation
Nach der installation kannst du im Terminal überprüfen ob alles korrekt installiert wurde:

```sh
rust --version
cargo --version
```

## IDE / Entwicklungsumgebung
Als IDE empfiehlt sich die Jetbrains RustRover IDE zu verwenden. Grundsätzlich aber users choice.
Ohne IDE können alle "Crates" mittels
```sh
cargo run
```
ausgeführt werden.
