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
# e.g. in directory Math/Geometry.dmd
module Math.Geometry
```

* Packages with
```diamond
import package_name.module
```
-> packagemanager “dimp”?

* Generic functions
```diamond
fn identity<T> : T = (val : T) {
    out val;
}
```

*Interface with Default-Implementation
```diamond
interface Printable {
    fn println : void = () {
        print("Printable");
    }
}
```

* Components combine fields and methods

* Components implement Interfaces
Comp Document implements Printable {
    fn println : void = () {
        print("overridden");
    }
}

