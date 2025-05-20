# Setup Rust
## MacOS
### Voraussetzung: C Compiler (Xcode Command Line Tools)

Rust benötigt einen **C-Compiler** für bestimmte Abhängigkeiten. Dieser ist normalerweise bereits vorhanden. Falls nicht, kann dieser so installiert werden:

1. Terminal öffnen
2. folgenden Befehl ausführen:
    ```sh
    xcode-select --install
    ```

---

### Rust installieren
Öffne ein Terminal und führe folgenden Befehl aus:
```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

---

### Verifikation
Nach der installation kannst du im Terminal überprüfen ob alles korrekt installiert wurde:

```sh
rust --version
cargo --version
```

### IDE / Entwicklungsumgebung
Als IDE empfiehlt sich die Jetbrains RustRover IDE zu verwenden. Grundsätzlich aber users choice.
Ohne IDE können alle "Crates" mittels
```sh
cargo run
```
im Ordner des jeweiligen Code-Beispiels ausgeführt werden.

## Windows
### Voraussetzung: Visual C++ Build Tools

Rust benötigt die Microsoft C++ Build Tools für Visual Studio 2017 oder neuer.

#### So installierst du die benötigten Build Tools

1. Besuche die offizielle Download-Seite von Visual Studio:  
   👉 https://visualstudio.microsoft.com/downloads/

2. Lade **Microsoft Visual Studio** oder die **Build Tools for Visual Studio** herunter und installiere sie.

3. Wähle während der Installation folgendes aus:  
   ✅ **„Desktopentwicklung mit C++“**

   Damit werden alle nötigen Komponenten für die Rust-Entwicklung installiert.

4. Wenn dein Windows **nicht auf Englisch** eingestellt ist, wähle zusätzlich aus:  
   ✅ **Englisch** unter **Sprachpakete**,  
   um Sprachprobleme mit bestimmten Toolchains zu vermeiden.

Weitere Informationen findest du in der offiziellen Dokumentation:  
🔗 https://rust-lang.github.io/rustup/installation/windows-msvc.html

---

### Rust installieren

1. Gehe zur offiziellen Rust-Installationsseite:  
   👉 https://www.rust-lang.org/learn/get-started

2. Lade `rustup-init.exe` herunter (32-Bit oder 64-Bit je nach Systemarchitektur).

3. Starte das Installationsprogramm und folge den Anweisungen.  
   ✅ Verwende die **Standard-Einstellungen**.

---

Nach der Installation kannst du in PowerShell oder der Eingabeaufforderung überprüfen, ob alles korrekt installiert wurde:

```sh
rustc --version
cargo --version
```

### IDE / Entwicklungsumgebung
Als IDE empfiehlt sich ebenfalls auf Windows die Jetbrains RustRover IDE zu verwenden. Grundsätzlich aber users choice.
Ohne IDE können alle "Crates" mittels
```sh
cargo run
```
im Ordner des jeweiligen Code-Beispiels ausgeführt werden.

# Aufbau Projekt
Im Ordner `src/` finden sich alle Code-Beispiele.

## Fokuspunkte
Die Fokuspunkte sind im Unterordner `src/focus-points/` aufzufinden.
Jeder Fokuspunkt besitzt über eine eigene Crate in einem jeweiligen Unterordner. Diese Unterordner beinhalten jeweils auch ein README, welches über den Fokuspunkt aufklären soll.

Die einzelnen Code-Beispiele können mit `cargo run` im jeweiligen Ordner gestartet werden.

## Übungsbeispiele
Die 3 gelösten Übungen aus dem PCP-Kurs können im Unterordner `src/exercises` gefunden werden und bestehen aus jeweils eigenen Crates, welche mit `cargo run` gestartet werden können.
Diese Crates beinhalten ebenfalls jeweils eine README datei, welche die Aufgabenstellung erläutert.

## Bericht & Präsentation
Der Bericht und die Folien der Präsentation sind im `docs/` Ordner vorzufinden.

# Honorable Mention
Wenn du Rust nun installiert hast und interesse an mehr hast, führe folgenden Befehl im Terminal deiner Wahl aus:
```sh
rustup docs --book
```
und geniesse das tolle Buch für deinen Einstieg in Rust!

---
See full project for cloning on [GitHub](https://github.com/kwstnr/hslu-pcp-rust)
