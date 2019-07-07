Johnston
========

[![Build Status](https://travis-ci.org/jcpst/johnston.svg?branch=master)](https://travis-ci.org/jcpst/johnston)

A library for working with JI pitch lattices. Very much a work in-progess.

Started in Common Lisp, then Clojure and back.  [old repo](https://github.com/jcpst/pitch-lattice)

Currently working on it as a Rust library.

Example
-------

```rust
extern crate johnston;
use johnston::*;

fn main() {
    let lattice = gen_lattice(&[3], 5);
    println!("{:#?}", lattice);
}
```

result:

```shell
[
    LatticeDimension {
        limit: 3,
        otonal: [
            Pitch {
                cents: 0.0,
                ratio: 1,
            },
            Pitch {
                cents: 701.95496,
                ratio: 3/2,
            },
            Pitch {
                cents: 203.90999,
                ratio: 9/8,
            },
            Pitch {
                cents: 905.8649,
                ratio: 27/16,
            },
            Pitch {
                cents: 407.81998,
                ratio: 81/64,
            },
        ],
        utonal: [
            Pitch {
                cents: 0.0,
                ratio: 1,
            },
            Pitch {
                cents: 498.0449,
                ratio: 4/3,
            },
            Pitch {
                cents: 996.0899,
                ratio: 16/9,
            },
            Pitch {
                cents: 294.13483,
                ratio: 32/27,
            },
            Pitch {
                cents: 792.1799,
                ratio: 128/81,
            },
        ],
    },
]
```
