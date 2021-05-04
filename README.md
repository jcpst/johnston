Johnston
========

[![Build Status](https://travis-ci.org/jcpst/johnston.svg?branch=master)](https://travis-ci.org/jcpst/johnston)
![Crates.io](https://img.shields.io/crates/v/johnston)

A library for working with JI pitch lattices. Very much a work in-progess.

Started in Common Lisp, then Clojure and back.  [old repo](https://github.com/jcpst/pitch-lattice)

Example
-------

### Generate a lattice

```rust
use johnston::Lattice;

fn main() {
    let lattice = Lattice::new(&[3, 5], 3);
    println!("{:?}", lattice);
}
```

<details>
    <summary>result</summary>

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
</details>

### Generate a scale

```rust
use johnston::Lattice;

fn main() {
    for pitch in Lattice::new(&[3, 5], 3).scale() {
        print!("{} ", pitch.cents);
    }
}
```

```
0 203.90999 386.3137 427.37253 498.04504 701.95496 772.6274 813.6863 996.09
```

