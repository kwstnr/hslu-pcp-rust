# Aufbau auf Stack-Machine mit Bäumen (Woche 7 – Aufgabe 4)

## Mathematische Ausdrücke als Bäume

Beispielausdruck: `(2 + 3) * 4`
→ Clojure-Darstellung:

```clojure
(defrecord Op [op left right])
(defrecord Val [value])
(def expr (Op. "*" (Op. "+" (Val. 2) (Val. 3)) (Val. 4)))
```

---

## a) Baum für `2 * (4 - 2) + (6 / 3)`

Zeichnen Sie den Ausdruck als Baum.

---

## b) Baum manuell erzeugen

Erzeugen Sie den Baum wie im Beispiel mit `Op.` und `Val.`-Knoten.

---

## c) Vergleich mit `parse`-Funktion

Nutzen Sie `parse` zum automatischen Erzeugen des Baums und vergleichen Sie ihn mit dem manuell erzeugten Baum mittels `=`.

---

## d) Funktion `eval-expr`

Implementieren Sie `eval-expr`, um den Baum rekursiv auszuwerten.
**Hinweise:**

* Ganzzahlige Berechnung genügt
* Division ebenfalls ganzzahlig

---

## e) Funktion `compile-expr`

Übersetzen Sie den Baum in ein Programm für die Stack-Maschine (`compile-expr`).
Testen Sie die Funktion mit mehreren Ausdrücken.
Vergleichen Sie das Ergebnis von `eval-expr` mit dem Resultat des Stack-Programms zur Kontrolle.
