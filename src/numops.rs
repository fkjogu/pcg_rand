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
 */
 
use extprim::u128::u128;

/// The types of numaric options that PCG needs to operate. 
/// Some day this will be replaced with Num-traits when they support
/// wrapping opts for everything, and when extprim supports those traits as
/// well.
pub trait PcgOps {
    fn wrap_mul(&self, rhs : Self) -> Self;
    fn wrap_add(&self, rhs : Self) -> Self;
}

/// Convert a value to a usize don't care about overflow etc
pub trait AsUsize {
    fn as_usize(&self) -> usize;
}

/// A trait that determines how many bits are in a type. 
pub trait BitSize {
    fn bits() -> usize;
}

/// Allows a type to become a type of a smaller value.
pub trait AsSmaller<T> {
    fn shrink(self) -> T;
}

//Implementations of the traits for basic types
macro_rules! basic_ops {
    ( $( $t:ty, $bits:expr);*) => {
        $(impl BitSize for $t {
            #[inline]
            fn bits() -> usize {
                $bits
            }
        }

        impl AsUsize for $t {
            fn as_usize(&self) -> usize {
                *self as usize
            }
        }
        
        impl PcgOps for $t {
            #[inline]
            fn wrap_mul(&self, rhs : $t) -> $t {
                self.wrapping_mul(rhs) 
            }
            
            #[inline]
            fn wrap_add(&self, rhs : $t) -> $t {
                self.wrapping_add(rhs)
            }
        }
            
        )*
    }
}

basic_ops!(
    u8, 8;
    u16, 16;
    u32, 32;
    u64, 64
);

macro_rules! smaller {
    ( $( $t:ty, $other:ty);*) => {
        $(
            impl AsSmaller<$other> for $t {
                fn shrink(self) -> $other {
                    self as $other
                }
            }
        )*       
    }
}

smaller!(
    u64, u32;
    u64, u16;
    u64, u8;
    u32, u16;
    u32, u8;
    u16, u8
);

impl AsSmaller<u64> for u128 {
    fn shrink(self) -> u64 {
        //Truncate the number
        self.low64()
    }
}

impl AsSmaller<u32> for u128 {
    fn shrink(self) -> u32 {
        //Truncate the number
        self.low64() as u32
    }
}

impl BitSize for u128 {
    fn bits() -> usize {
        128
    }
}

impl AsUsize for u128 {
    fn as_usize(&self) -> usize {
        self.low64() as usize
    }
}

impl PcgOps for u128 {
    #[inline]
    fn wrap_mul(&self, rhs : u128) -> u128 {
        self.wrapping_mul(rhs) 
    }
    
    #[inline]
    fn wrap_add(&self, rhs : u128) -> u128 {
        self.wrapping_add(rhs)
    }
}