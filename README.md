# DiamondCompiler
(*.dmd*)

Compiler for the diamond lang, a compiled statically typed general purpose language.
Diamond will focus on safe and fast parallel execution in the future. 
It is (currently) compiled to x86_64 assembly.

I am always looking for new (or better) features to include. **Feel free to contribute!!**


## functionality 

Examples are in the *tests* folder.

## upcoming

* more data-types
* structs holding data
```diamond
struct rect {
    width :int32,
    height :int32
}
```
* Each file starts with it's Modulname:
```diamond
# e.g. in directory Math/Geometry.dmd
module Math.Geometry
```
* Module and Namespaces:
```diamond
use Math.Geometry as Geo;
```
and
```diamond
namespace Algebra {
    # Here is defined namespace
}
```


## later on
> Neither fixed syntactically nor content-wise!

* async/await (just on immutable data)
* mutable variables
```diamond
mut x :int32 = 100;
x = x + 10; 

```
* (parallel) loops
```diamond

for i to 20 {
    # do smth.
}

parallel for j to 10 {
    # do smth.
}

```
* threads
* Pattern matching
* Option types
* define types with 
```diamond
# union
type t = some(int) | none(int);
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

* Interface with Default-Implementation
```diamond
interface Printable {
    fn println : void = () {
        print("Printable");
    }
}
```

* Components combine fields and methods, but are not accessable from outside itself (structs are).
Components implement Interfaces:
```diamond
Comp Document implements Printable {
    fn println : void = () {
        print("overridden");
    }
}
```
