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

pub trait Multiplier<Itype> {
    fn build() -> Self;
    
    fn multiplier(&self) -> Itype;
}

use ::numops::PcgConsts;


pub struct DefaultMultiplier;

impl<Itype: PcgConsts> Multiplier<Itype> for DefaultMultiplier {
    fn build() -> DefaultMultiplier {
        DefaultMultiplier
    }
    
    #[inline]
    fn multiplier(&self) -> Itype {
        Itype::default()
    }
}

// make_default_mul!(
//     u8 => 141u8;
//     u16 => 12829u16;
//     u32 => 747796405u32;
//     u64 => 6364136223846793005u64
// );

pub struct McgMultiplier;

impl<Itype: PcgConsts> Multiplier<Itype> for McgMultiplier {
    fn build() -> McgMultiplier {
        McgMultiplier
    }
    
    #[inline]
    fn multiplier(&self) -> Itype {
        Itype::mcg()
    }
}

// make_mcg_mul!(
//     u8 => 217u8;
//     u16 => 62169u16;
//     u32 => 277803737u32;
//     u64 => 12605985483714917081u64
// );
