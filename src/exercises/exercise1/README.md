# Fibonacci: Rekursion und Memoization (Woche 3 – Aufgabe 1 & 2)

## 1. Endrekursive Fibonacci-Berechnung mit Ein- und Ausgabe

In der Vorlesung wurde eine endrekursive Prolog-Implementierung für Fibonacci-Zahlen besprochen.  
Implementieren Sie nun ein Prädikat `io_fib/0`, so dass der Benutzer zuerst eine Zahl eingeben muss und danach die dazugehörende Fibonacci-Zahl ausgegeben wird.

## 2. Memoization: Optimierung der Fakultätsberechnung durch Assertions

Mithilfe von Assertions können neue Fakten in der Wissensdatenbank gespeichert werden. Dies kann verwendet werden, um Berechnungen durch Speichern von Zwischenergebnissen zu optimieren.

### a) Fakultätsberechnung mit asserta/1

* Erweitern Sie das Prädikat `fak/2`, so dass bereits berechnete Werte mit `asserta/1` in `fak_as/2` gespeichert werden.
* Prüfen Sie bei einem Aufruf von `fak/2`, ob bereits ein passender Wert in `fak_as/2` vorhanden ist.
* Bei gefundenem Wert soll eine Konsolenausgabe erfolgen:

  ```prolog
  Hinweis: Fakultät von 7 war gespeichert.
  ```

### b) Löschen gespeicherter Werte mit fak\_clear/0

* Implementieren Sie ein Prädikat `fak_clear/0`, das mit `retractall/1` alle gespeicherten Werte von `fak_as/2` löscht.
* Konsolenausgabe bei Löschung:

  ```prolog
  Hinweis: Alle gespeicherten Werte wurden gelöscht.
  ```

**Beispielhafte Ablaufsequenz in der Prolog-Konsole:**

```prolog
?- fak(7, R).
?- fak(7, R).
?- fak_clear.
```
