#[macro_export]
macro_rules! interval {
    ($name:ident $num:tt/$den:tt) => {
        interval!($name, $num, $den, stringify!($num), stringify!($den));
    };

    ($name:ident, $num:tt, $den:tt, $snum:expr, $sden:expr) => {
        #[allow(dead_code)]
        #[doc = $snum]
        #[doc = "/"]
        #[doc = $sden]
        pub const $name: (i32, i32) = ($num, $den);
    };

    ($name:ident $num:tt/$den:tt $notes:tt) => {
        interval!($name, $num, $den, stringify!($num), stringify!($den), $notes);
    };

    ($name:ident, $num:tt, $den:tt, $snum:expr, $sden:expr, $notes:expr) => {
        #[allow(dead_code)]
        #[doc = $snum]
        #[doc = "/"]
        #[doc = $sden]
        #[doc = " ("]
        #[doc = $notes]
        #[doc = ")"]
        pub const $name: (i32, i32) = ($num, $den);
    }
}

interval! { TONIC  1/1 }
interval! { OCTAVE 2/1 }

interval! { SYNTONIC_COMMA        81/80 }
interval! { SIXTY_FIFTH_HARMONIC  65/64 }
interval! { PTOLEMYS_ENHARMONIC   56/55 }
interval! { INFERIOR_QUARTER_TONE 46/45 "3 to the 12th/2 to the 19th" }
interval! { SEPTIMAL_COMMA        36/35 }
interval! { UNDECIMAL_COMMA       33/32 }

interval! { C  1/1 }
interval! { D  9/8 }
interval! { E  5/4 }
interval! { F  4/3 }
interval! { G  3/2 }
interval! { A  5/3 }
interval! { B 15/8 }
