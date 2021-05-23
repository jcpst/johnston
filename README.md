Johnston
========

[![ci build](https://github.com/jcpst/johnston/actions/workflows/rust.yml/badge.svg)](https://github.com/jcpst/johnston/actions/workflows/rust.yml)
![Crates.io](https://img.shields.io/crates/v/johnston)

A library for working with JI pitch lattices. Very much a work in-progess.

Started in Common Lisp, then Clojure and back.  [old repo](https://github.com/jcpst/pitch-lattice)

Example
-------

### Generate a lattice

```rust
use johnston::{lattice::LatticeDimension, pitch::Pitch};

fn main() {
    let pitch = Pitch::new((5, 4));
    let lattice_dimension = LatticeDimension::new(pitch);
    let notes = lattice_dimension.take(6);
	
    for note in notes {
        println!("{:?}", note);
    }
}
```

```shell
Pitch { cents: 386.3137, ratio: Ratio { numerator: 5, denominator: 4 }, limit: 5, ordinal: Otonal }
Pitch { cents: 772.6274, ratio: Ratio { numerator: 25, denominator: 16 }, limit: 5, ordinal: Otonal }
Pitch { cents: 1158.9412, ratio: Ratio { numerator: 125, denominator: 64 }, limit: 5, ordinal: Otonal }
Pitch { cents: 345.25482, ratio: Ratio { numerator: 625, denominator: 512 }, limit: 5, ordinal: Otonal }
Pitch { cents: 731.56854, ratio: Ratio { numerator: 3125, denominator: 2048 }, limit: 5, ordinal: Otonal }
Pitch { cents: 1117.8822, ratio: Ratio { numerator: 15625, denominator: 8192 }, limit: 5, ordinal: Otonal }
```
