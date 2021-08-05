#[macro_export]
macro_rules! interval {
    ($name:ident $num:tt/$den:tt) => {
        #[allow(dead_code)]
        #[doc = concat!(stringify!($num), "/", stringify!($den))]
        pub const $name: (i32, i32) = ($num, $den);
    };

    ($name:ident $num:tt/$den:tt $notes:tt) => {
        #[allow(dead_code)]
        #[doc = concat!(stringify!($num), "/", stringify!($den), " - _", $notes, "_")]
        pub const $name: (i32, i32) = ($num, $den);
    };
}

interval! { TONIC                 1/1   }
interval! { SUPERPARTICULAR_CENT_APPROXIMATION 1732/1731 }
interval! { SCHISMA               32805/32768 "3 to the 8th/2 to the 12/th x 5/8" }
interval! { HARMONIC_129          129/128 }
interval! { SUPERPARTICULAR_PYTHAGOREAN_COMMA_APPROXIMATION 74/73 }
interval! { HARMONIC_65           65/64 }
interval! { PTOLEMYS_ENHARMONIC   56/55 }
interval! { INFERIOR_QUARTER_TONE 46/45   "Ptolemy" }
interval! { HARMONIC_131          131/128 }
interval! { DIMINISHED_SECOND     128/125 "16/15 x 24/25" }
interval! { ENHARMONIC_DIESIS     525/512 "Avicenna" }
interval! { SUPERIOR_QUARTER_TONE 39/38   "Eratosthenes" }
interval! { SUPERIOR_QUARTER_TONE_2 36/35 "Archytas" }
interval! { ET_QUARTER_TONE_APPROXIMATION 35/34 }
interval! { HARMONIC_33           33/32 }
interval! { INFERIOR_QUARTER_TONE_2 32/31 "Didymus" }
interval! { SUPERIOR_QUARTER_TONE_3 31/30 "Didymus" }
interval! { INFERIOR_QUARTER_TONE_3 28/27 "Archytas" }
interval! { HARMONIC_133          133/128 }
interval! { ONE_THIRD_TONE        26/25   "Avicenna" }
interval! { MINOR_FIVE_LIMIT_HALF_STEP 25/24 }
interval! { HARMONIC_67           67/64 }
interval! { HARD_HALF_STEP        22/21   "Ptolemy, Avicenna, Safiud" }
interval! { SEPTIMAL_SEMITONE     21/20 }
interval! { PYTHAGOREAN_HALF_STEP 256/243 }
interval! { LIMMA_ASCENDANT       135/128 }
// TODO: more intervals
interval! { OCTAVE                2/1   }


interval! { C  1/1 }
interval! { D  9/8 }
interval! { E  5/4 }
interval! { F  4/3 }
interval! { G  3/2 }
interval! { A  5/3 }
interval! { B 15/8 }

interval! { SYNTONIC_COMMA        81/80 }
interval! { PYTHAGOREAN_COMMA     531441/524228 "3 to the 12th/2 to the 19th" }
interval! { SEPTIMAL_COMMA        36/35 }
interval! { UNDECIMAL_COMMA       33/32 }