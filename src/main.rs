extern crate pcg_rand;
extern crate rand;

use rand::{Rng, OsRng};
use pcg_rand::{Pcg32Basic, Pcg32_unique};


fn main() {
    let mut rng = Pcg32Basic::new_unseeded();

    // print a bunch of random numbers
    println!("Here is the generator recovering from a (0,0) initialization: ");
    for _ in 0..7 {
        println!("{: ^25}|{: ^25}|{: ^25}", rng.gen::<u32>(), rng.gen::<u32>(), rng.gen::<u32>());
    }

    let mut rng : Pcg32Basic = OsRng::new().unwrap().gen();
    println!("\nHere is the generator with random seed and increment: ");
    for _ in 0..7 {
        println!("{: ^25}|{: ^25}|{: ^25}", rng.gen::<u32>(), rng.gen::<u32>(), rng.gen::<u32>());
    }

    //Later we will do party tricks
    //TODO: Implement the really big generators to get the party started

    println!("\nHere we show off what two unique stream generators with the same seed can do");
    println!("Example Code:
    let mut urng1 = Pcg32_unique::new_unseeded();
    let mut urng2 = Pcg32_unique::new_unseeded();
    ");
    let mut urng1 = Pcg32_unique::new_unseeded();
    let mut urng2 = Pcg32_unique::new_unseeded();
    println!("{: ^25}|{: ^25}", "Generator 1", "Generator2");
    for _ in 0..25 {
        println!("{: ^25}|{: ^25}", urng1.gen::<u32>(), urng2.gen::<u32>());
    }
    println!("The RNGs use their location as a sequence so they diverge quickly");
}
