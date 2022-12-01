# Projektarbeit MiniCompiler
## Aufgabenstellung
Umsetzung dieses [C++ Mini-Compilers](https://sulzmann.github.io/SoftwareProjekt/schein-neu.html#(8)) nach Rust.</br>
Dieser besteht aus drei Teilen: 
* Tokenizer: Verwandelt den Input text in für den Parser verständliche Tokens
* AST (Arithmetic Syntax Tree): Ermöglicht die Repräsentation und Verarbeitung von arithmetischen Ausdrücken durch eine Baumstruktur
* Parser: Verarbeitet den Input mithilfe des Tokenizers und versucht über die Regeln einer Grammatik daraus einen AST zu bauen

## Tokenizer
Die Basis für die Tokenization ist das enum TokenT, welches alle möglichen Token Arten beinhaltet.
```
#[derive(Debug, PartialEq)] 
pub enum TokenT {       
    EOS, ZERO, ONE, TWO, OPEN, CLOSE, PLUS, MULT
} 
```
Man sieht über der Definition des Enums ein ``#[derive(Debug, PartialEq)]``, das bewirkt die automatische Implementierung der in der Klammer genannten Traits.</br>
Dabei ermöglicht ``PartialEq`` Vergleichsoperationen und ``Debug`` ermöglicht die Programmierung-seitige Formatierung, was für Tests gebraucht wird.</br>
Um die User-seitige Formatierung und damit ``TokenT.to_string()`` zu ermöglichen muss noch manuell das Trait ``std::fmt::Display`` implementiert werden.#


