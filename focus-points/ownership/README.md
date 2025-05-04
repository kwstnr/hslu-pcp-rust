# Ownership, Borrowing & Move Semantik beispiele
In diesem Rust Projekt, können Beispiele über die Ownership Konzepte von Rust eingesehen werden.

## Was ist Ownership in Rust
Ownership ist ein Set an Regeln, welche definieren, wie Rust den genutzten Speicher verwaltet.
Hervorzuheben ist hierbie, dass Rust über keinene Garbage-Collector verfügt. Trotzdem muss Speicher nicht manuell alloziert und dealloziert werden.
Speicher wird mittels dem Ownership-System verwaltet.

## Ownership-Regeln
- Jeder Wert in Rust hat einen `owner`, also einen Besitzer.
- Jeder Wert kann nur einen `owner` zur gleichen Zeit haben.
- Wenn der `owner` out-of-scope geht, wird der Wert gedropped.

## Der Stack- und der Heap-Speicher
In Rust ist es für das Verhalten des Systems von grosser Bedeutung, ob ein Wert auf dem Stack oder dem Heap liegt.
Werte auf dem Stack werden über call-by-value als Paremter übergeben, wobei für Werte auf dem Heap eine Referenz übergeben wird.
Dies ist wichtig um zu verstehen, wie Ownership unterschiedlich angewandt werden muss für verschiedene Arten von Werten.
