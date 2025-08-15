# DiamondCompiler
(*.dmd*)

Compiler for the diamond lang, a compiled statically typed general purpose language.
Diamond will focus on safe parallel execution in the future. 
It is (currently) compiled to x86_64 assembly.


## functionality 

Examples are in the *tests* folder.

## upcoming

* more data-types
* interfaces 
* components
* generic functions

## later on

* Parallelität durch async/await nur Zugriff auf immutable.
* Pattern matching
* Option types
* define types with 
```diamond
# union
type t = some(int) | none(int);
```
and 
```diamond
# records
type t = { a :int, b :int } 
```
Module und Namespaces:
    use Math.Geometry as Geo;
Und
    namespace Algebra {
        # hier definierter namespace
    }
Jedes File startet mit Modulnamen:
    module Math.Geometry
In Verzeichnis Math/Geometry.dmd
Packages mit 
import package_name.module
packagemanager “dimp”?
Generische Funktionen
fn identity<T> : T = (val : T) {
    out val;
}

Interface mit Default-Implementierung
interface Printable {
    fn println : void = () {
        print("Printable");
    }
}

Components implementieren Interfaces
Comp Document implements Printable {
    fn println : void = () {
        print("overridden");
    }
}
Component = Felder und Methoden
