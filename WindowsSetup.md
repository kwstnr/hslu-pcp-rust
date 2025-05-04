# Windows-Setup für Rust

## Voraussetzung: Rust Visual C++ Build Tools

Rust benötigt die Microsoft C++ Build Tools für Visual Studio 2017 oder neuer.

### So installierst du die benötigten Build Tools

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

## Rust installieren

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
