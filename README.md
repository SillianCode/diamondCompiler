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

* async/await (just on immutable data)
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

* Module and Namespaces:
```diamond
use Math.Geometry as Geo;
```
Und
```diamond
namespace Algebra {
    # Here is defined namespace
}
```
* Each file starts with it's Modulname:
```diamond
module Math.Geometry
```
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
