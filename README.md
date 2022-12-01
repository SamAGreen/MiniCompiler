# Projektarbeit MiniCompiler
## Aufgabenstellung
Umsetzung dieses [C++ Mini-Compilers](https://sulzmann.github.io/SoftwareProjekt/schein-neu.html#(8)) nach Rust.</br>
Dieser besteht aus drei Teilen: 
* Tokenizer: Verwandelt den Input text in für den Parser verständliche Tokens
* AST (Arithmetic Syntax Tree): Ermöglicht die Repräsentation und Verarbeitung von arithmetischen Ausdrücken durch eine Baumstruktur
* Parser: Verarbeitet den Input mithilfe des Tokenizers und versucht über die Regeln einer Grammatik daraus einen AST zu bauen

## Tokenizer
### TokenT
Die Basis für die Tokenization ist das enum TokenT, welches alle möglichen Token Arten beinhaltet.
```
#[derive(Debug, PartialEq)] 
pub enum TokenT {       
    EOS, ZERO, ONE, TWO, OPEN, CLOSE, PLUS, MULT
} 
```
Man sieht über der Definition des Enums ein ``#[derive(Debug, PartialEq)]``, das bewirkt die automatische Implementierung der in der Klammer genannten Traits.</br>
Dabei ermöglicht ``PartialEq`` Vergleichsoperationen und ``Debug`` ermöglicht die Programmierung-seitige Formatierung, was für Tests gebraucht wird.</br>
Um die User-seitige Formatierung und damit ``TokenT.to_string()`` zu ermöglichen muss noch manuell das Trait ``std::fmt::Display`` implementiert werden.

### Tokenize
Hier liegt die Businesslogik des Tokenizers. ``Tokenize``: 
* Mit ``next()`` wird der nächste Token aus dem Eingabestring generiert
* Mit ``scan()`` wird der ganze Eingabestring verarbeitet und als Vector von ``TokenT`` zurück gegeben
* Mit ``show()`` wird der ganze Eingabestring zu einem Strichpunkt-separierten String von Tokens konvertiert



### Tokenizer
Ist eine Wrapperklasse für ``Tokenize`` 
