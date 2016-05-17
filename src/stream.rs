/*
 * PCG Random Number Generation for Rust
 *
 * Copyright 2015 John Brooks <robojeb@robojeb.xyz>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * This work is derived from the implementation PCG RNG for C++ by
 * Melissa O'Neill.
 *
 * For additional information about the PCG random number generation scheme,
 * including its license and other licensing options, visit
 *
 *     http://www.pcg-random.org
 */

use ::numops::{PcgConsts, PcgOps};
use rand::{Rng, Rand};

pub trait Stream<Itype> {
    fn build() -> Self;
    
    fn set_stream(&mut self, _stream_seq : Itype){
        panic!("Stream setting unimplemented for this stream type");
    }

    fn increment(&self) -> Itype;

    fn get_stream(&self) -> Itype;
}

pub struct OneSeqStream;

impl<Itype: PcgConsts> Stream<Itype> for OneSeqStream {
    fn build() -> Self {
        OneSeqStream
    }
    
    fn increment(&self) -> Itype {
        Itype::stream()
    }
    
    fn get_stream(&self) -> Itype {
        Itype::stream()
    }
}

impl Rand for OneSeqStream {
    fn rand<R: Rng>(_rng: &mut R) -> Self {
        OneSeqStream
    }
}

pub struct NoSeqStream;

impl<Itype: PcgOps> Stream<Itype> for NoSeqStream {
    fn build() -> Self {
        NoSeqStream
    }
    
    fn increment(&self) -> Itype {
        Itype::zero()
    }
    
    fn get_stream(&self) -> Itype {
        Itype::zero()
    }
}

impl Rand for NoSeqStream {
    fn rand<R: Rng>(_rng: &mut R) -> Self {
        NoSeqStream
    }
}

pub struct SpecificSeqStream<Itype> {
    inc : Itype
}

impl<Itype: PcgConsts + Clone> Stream<Itype> for SpecificSeqStream<Itype> {
    fn build() -> Self {
        SpecificSeqStream {
            inc : Itype::stream(),
        }
    }
    
    fn set_stream(&mut self, stream_seq : Itype) {
        self.inc = stream_seq;
    }
    
    fn increment(&self) -> Itype {
        self.inc.clone()
    }
    
    fn get_stream(&self) -> Itype {
        self.inc.clone()
    }
}

impl<Itype: Rand> Rand for SpecificSeqStream<Itype> {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        SpecificSeqStream {
            inc : rng.gen(),
        }
    }
}

// macro_rules! specific_new {
//     ( $($t:ty => $e:expr);*) => {
//         $(impl SpecificSeqStream<$t> {
//             #[allow(dead_code)]
//             pub fn new() -> SpecificSeqStream<$t> {
//                 //We use a default good sequence so a default initialized
//                 //version of SpecificSeqStream is the same as OneSeqStream
//                 SpecificSeqStream{inc: $e}
//             }

//             #[allow(dead_code)]
//             pub fn new_with_value(v : $t) -> SpecificSeqStream<$t> {
//                 SpecificSeqStream{inc: v}
//             }
//         }

//         )*
//     }
// }

// specific_new!(
//     u8  => 77u8; //These are probably useless in rust
//     u16 => 47989u16; // ^
//     u32 => 2891336453u32;
//     u64 => 1442695040888963407u64
// );



// macro_rules! make_specific_seq {
//     ( $($t:ty),* ) => {
//         $(impl Stream<$t> for SpecificSeqStream<$t> {
//             fn set_stream(&mut self, stream_seq : $t) {
//                 self.inc = (stream_seq << 1) | 1;
//             }

//             #[inline]
//             fn increment(&self) -> $t {
//                 self.inc
//             }

//             fn get_stream(&self) -> $t {
//                 self.inc
//             }
//         })*
//     }
// }

// use std::cell::Cell;

// pub struct UniqueSeqStream<IType> {
//     inc : Cell<Option<IType>>,
// }

// impl<IType: Copy> UniqueSeqStream<IType> {
//     pub fn new() -> UniqueSeqStream<IType> {
//         UniqueSeqStream {
//             inc : Cell::new(None)
//         }
//     }
// }

// macro_rules! make_unique_seq {
//     ( $($t:ty),* ) => {
//         $(impl Stream<$t> for UniqueSeqStream<$t> {
//             #[inline]
//             fn increment(&self) -> $t {
//                 match self.inc.get() {
//                     None => {
//                         let inc = self as *const UniqueSeqStream<$t>;
//                         let inc = inc as $t | 1;
//                         self.inc.set(Some(inc));
//                         inc
//                     },
//                     Some(inc) => inc,
//                 }
//             }

//             fn get_stream(&self) -> $t {
//                 match self.inc.get() {
//                     None => {
//                         let inc = self as *const UniqueSeqStream<$t>;
//                         let inc = inc as $t | 1;
//                         self.inc.set(Some(inc));
//                         inc
//                     },
//                     Some(inc) => inc,
//                 }
//             }
//         })*
//     }
// }


// //For use with MCG
// pub struct NoSeqStream<IType>{
//     phantom    : PhantomData<IType>
// }

// impl<IType> NoSeqStream<IType> {
//     pub fn new() -> NoSeqStream<IType> {
//         NoSeqStream{
//             phantom : PhantomData::<IType>,
//         }
//     }
// }

// macro_rules! make_no_seq {
//     ( $($t:ty),* ) => {
//         $(impl Stream<$t> for NoSeqStream<$t> {
//             #[inline]
//             fn increment(&self) -> $t {
//                 0
//             }

//             fn get_stream(&self) -> $t {
//                 0
//             }
//         })*
//     }
// }

// //Make the implementations for all the various sequence types
// make_one_seq!(
//     u8  => 77u8; //These are probably useless in rust
//     u16 => 47989u16; // ^
//     u32 => 2891336453u32;
//     u64 => 1442695040888963407u64
// );
// make_specific_seq!(u8,u16,u32,u64);
// make_unique_seq!(u8, u16, u32, u64);
// make_no_seq!(u8, u16, u32, u64);
