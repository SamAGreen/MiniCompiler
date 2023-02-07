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
* Mit ``scan()`` wird der ganze Eingabestring verarbeitet und als Vector von ``TokenT`` zurück gegeben. 
Wobei ich die Methode leicht abgeändert hat, damit sie ``pos`` nach Beendung der Methode auf den gleichen Wert wie davor zurücksetzt
* Mit ``show()`` wird der ganze Eingabestring zu einem Strichpunkt-separierten String von Tokens konvertiert



### Tokenizer
Ist eine Wrapperklasse für ``Tokenize``, die zu dessen Funktionalität auch noch den derzeitigen Token speichert.
Außerdem habe ich einen pseudo-Konstruktor ``tokenizer(s: &str) -> Tokenizer`` geschrieben, um die Erzeugung von Tokenizern zu vereinfachen.

### Tests
Weil Rust standardmäßig die Möglichkeit bietet automatisierte Tests zu schreiben, habe ich ein paar parallel zur Implementierung geschrieben.
Natürlich um sicherzustellen, dass mein Code auch funktioniert und weil ich es ausprobieren wollte.

## AST (Arithmetic Syntax Tree)
Der AST ist ein Weg die arithmetischen Ausdrücke in einer Baumstruktur dazustellen.</br> 
Anfangs habe ich versucht den AST mit einem Trait, separaten Structs und Generics zu implementieren. 
Dies erwies sich aber problematisch wegen dynamischen Speicherbedarf und wurde zu komplex um eine sinnvolle Lösung zu sein.</br>
Meine jetzige Lösung basiert auf dem Fakt, dass ``enums`` und Pattern-matching mit ``match`` in Rust relativ mächtig sind. 

### Enum
```
pub enum Exp {
    Int {
        val: i32
    },
    Plus {
        e1: Box<Exp>,
        e2: Box<Exp>,
    },
    Mult {
        e1: Box<Exp>,
        e2: Box<Exp>,
    },
}
```
Jetzt hat man das enum ``Exp``, dass die structs:
* ``Int`` für einfache integer Werte
* ``Plus`` das zwei weitere Expressions beinhaltet, die miteinander addiert werden
* ``Mult`` das zwei weitere Expressions beinhaltet, die miteinander multipliziert werden

beinhaltet

Dabei ist es noch wichtig zu bemerken, dass in Rust die Größe von Typen auf dem Stack zur Compilezeit klar sein muss.
Das ist bei der Grundidee von rekursiven Datentypen, wie hier, nicht möglich:
```
Mult {
        e1: Exp,
        e2: Exp,
    },
```
Da die Größe von diesem struct unbekannt und potenziell unendlich ist, sind diese Arten von Ausdrücken illegal.
Daher wird das Keyword ``Box<T>`` benutzt, das ist ein Pointer auf den Heap und hat daher eine feste Datengröße. 
Da das struct dann aber auf dem Heap gespeichert wird, kann die Größe dynamisch sein.

### Funktionalität
Die Funktionalität der Ausdrücke soll über nur der Speicherung der Struktur hinaus gehen:
* Sich selbst auswerten, dies wird mit ``eval`` getan
* Sich in einem String ausgeben, dies wird mit ``pretty`` getan

Diese müssen natürlich abhängig von der Art des Ausdrucks variieren.
In einer Sprache mit Polymorphismus und Vererbung würden die verschiedenen Ausdrücke alle die gleiche Methode, mit verschiedener Funktionalität haben, das funktioniert in Rust nicht so. </br>
Hier gibt es jeweils nur eine Methode für das enum ``Exp``, das abhängig von der Art des Ausdrucks, der die Methode aufruft, was anderes macht. Das wird mit dem Keyword ``match`` erreicht. Hier in ``eval``:
```
pub fn eval(&self) -> i32 {
        return match self {
            Exp::Int { val } => { *val }
            Exp::Plus { e1, e2 } => { e1.eval() + e2.eval() }
            Exp::Mult { e1, e2 } => { e1.eval() * e2.eval() }
        };
    }
```
## Parser
Der Parser benutzt den ``Tokenizer`` und diese Grammatik:
```
E => T E'
E' => + T E'
T => F T'
T' => * F T'
F => N | (E)
```
um aus einem syntaktisch korrekten String einen validen AST zu bauen. </br>
Die grammatikalischen Regeln werden in den Methoden des Parsers abgebildet, somit wird aus ``T' => * F T'`` 
```
fn parse_t2(&mut self, left: Box<Exp>) -> Option<Box<Exp>> {
        if self.t.token == TokenT::MULT { 
            self.t.next_token();            

            let t = self.parse_f();
            return if let Some(right) = t {
                self.parse_t2(Box::new(Exp::Mult { e1: left, e2: right }))
            } else {                        
                t
            }
        }
        Some(left)
    }
```
Rückgabe Werte werden außerdem mit ``Option<T>`` gewrapped, welches ein Weg ist potenziell undefinierte Werte zu repräsentieren. 
Diese können entweder ``Some`` sein und einen Wert beinhalten, oder ``None`` sein und keinen Wert beinhalten.
`Option<Box<T>>`` ist damit Rusts Pendant zu Nullpointern.

## Quellen
Um ein Gefühl für die Sprache zu kriegen habe ich mich anfangs durch die [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/) Beispiele durchgeklickt. </br>
Die [Dokumentation](https://doc.rust-lang.org/std/index.html) war sehr hilfreich. Aber am meisten hat mir der Rust Compiler selbst geholfen, dieser hat nämlich sehr gute Hinweise bei Fehlern, die meistens das Problem direkt lösten.
