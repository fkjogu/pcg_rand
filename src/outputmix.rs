pub trait OutputMixin<Itype, Xtype> {
    fn output(&self, state : Itype) -> Xtype;
}

pub struct XshRsMixin;

macro_rules! make_Xsh_Rs_mixins {
    ( $( $it:ty, $xt:ty, $ibits:expr, $xbits:expr);*) => {
        $(impl OutputMixin<$it, $xt> for XshRsMixin {
            fn output(&self, state : $it) -> $xt {
                let mut state = state;

                const bits : usize= $ibits;
                const xtypebits : usize= $xbits;
                const sparebits : usize = bits - xtypebits;
                //Have to use "let" right now because if complains when used in a const context
                let opbits : usize = if sparebits-5 >= 64 { 5 } else
                             if sparebits-4 >= 32 { 4 } else
                             if sparebits-3 >= 16 { 3 } else
                             if sparebits-2 >= 4  { 2 } else
                             if sparebits-1 >= 1  { 1 } else { 0 };
                let mask = (1 << opbits) - 1;
                let maxrandshift = mask;
                let topspare = opbits;
                let bottomspare = sparebits - topspare;
                let xshift = topspare + (xtypebits+maxrandshift)/2;
                //Now we start real computation. Everything above was constexpr in the
                //C++ code originally
                let rshift = if opbits != 0 { (state >> (bits - opbits)) as usize & mask }
                    else { 0 };

                state ^= state >> xshift;
                (state >> (bottomspare - maxrandshift + rshift)) as $xt
            }
        }
        )*
    }
}

make_Xsh_Rs_mixins!(
    u16, u8, 16, 8;
    u32, u16, 32, 16;
    u64, u32, 64, 32
);

pub struct XshRrMixin;

macro_rules! make_Xsh_Rr_mixins {
    ( $( $it:ty, $xt:ty, $ibits:expr, $xbits:expr);*) => {
        $(impl OutputMixin<$it, $xt> for XshRrMixin {
            fn output(&self, state : $it) -> $xt {
                let mut state = state;

                const bits : usize= $ibits;
                const xtypebits : usize= $xbits;
                const sparebits : usize = bits - xtypebits;
                //Have to use "let" right now because if complains when used in a const context
                let wantedopbits : usize = if xtypebits >= 128 { 7 } else
                             if xtypebits >= 64 { 6 } else
                             if xtypebits >= 32 { 5 } else
                             if xtypebits >= 16 { 4 } else { 3 };

                let opbits = if sparebits >= wantedopbits { wantedopbits } else { sparebits };
                let amplifier = wantedopbits - opbits;
                let mask = (1 << opbits) - 1;
                let topspare = opbits;
                let bottomspare = sparebits - topspare;
                let xshift = (topspare + xtypebits)/2;

                //Begin math from the C++
                let rot = if opbits != 0 { (state >> (bits - opbits)) as usize & mask } else {0};
                let amprot = (rot << amplifier) & mask;
                state ^= state >> xshift;
                let result : $xt = (state >> bottomspare) as $xt;
                result.rotate_right(amprot as u32)
            }
        }
        )*
    }
}

make_Xsh_Rr_mixins!(
    u16, u8, 16, 8;
    u32, u16, 32, 16;
    u64, u32, 64, 32
);
