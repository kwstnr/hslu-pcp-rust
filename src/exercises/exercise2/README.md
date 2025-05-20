# Stack-Machine (Woche 6 – Aufgabe 6)

## Beschreibung

Eine einfache Programmiersprache mit den Befehlen `LOAD x`, `ADD`, `MUL`. Die Stack-Maschine arbeitet mit einem Stack aus natürlichen Zahlen:

| Befehl | Beschreibung                                               |
| ------ | ---------------------------------------------------------- |
| LOAD x | Zahl x auf den Stack legen (x ist positive natürliche Zahl)                 |
| ADD    | Zwei oberste Zahlen addieren, Ergebnis auf den Stack legen |
| MUL    | Zwei oberste Zahlen multiplizieren, Ergebnis auf den Stack |

Beispiel: `(2 + 3) * 4` →

```clojure
["LOAD 2" "LOAD 3" "ADD" "LOAD 4" "MUL"]
```

---

## a) Stack-Zustand visualisieren

Zeichnen Sie den Zustand des Stacks nach jeder Anweisung im obigen Beispielprogramm.

---

## b) Stack-Programm für `(4 + 5) * (2 + 3)`

Geben Sie ein passendes Stack-Programm an.

---

## c) Funktion `run-command`

Schreiben Sie eine Funktion `run-command`, die einen Stack und einen Befehl erhält und den neuen Stack-Zustand zurückgibt.

**Beispiele:**

```clojure
(run-command [] "LOAD 1")           ; => (1)
(run-command [1 2] "LOAD 3")        ; => (3 1 2)
(run-command [1 2 9 9 9] "ADD")     ; => (3 9 9 9)
(run-command [2 3 9 9 9] "MUL")     ; => (6 9 9 9)
```

---

## d) Funktion `run-program`

Schreiben Sie eine Funktion, die ein komplettes Stack-Programm ausführt und das oberste Stack-Element am Ende zurückgibt.

**Beispiel:**

```clojure
(run-program ["LOAD 2" "LOAD 3" "ADD" "LOAD 4" "MUL"]) ; => 20
```

---

## e) Erweiterung: SUB und DIV

* Fügen Sie `SUB` (Subtraktion) und `DIV` (ganzzahlige Division) hinzu.
* Stack-Programm für `5 * (6 / (7 - 4))` angeben und ausführen.

**Beispiel:**

```clojure
(run-command [4 7 9 9 9] "SUB") ; => (3 9 9 9)
```