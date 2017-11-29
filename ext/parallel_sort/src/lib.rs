extern crate rayon;

#[macro_use]
extern crate ruru;

use rayon::prelude::*;
use ruru::{Array, Class, Fixnum, Object};

class!(Integers);

methods! {
    Integers,
    _itself,

    fn sort(input: Array) -> Array {
        let input = input.unwrap();
        let length = input.length();

        let mut vector = (0..length).fold(Vec::with_capacity(length), |mut acc, i| {
            let number = unsafe { input.at(i as i64).to::<Fixnum>().to_i64() };

            acc.push(number);

            acc
        });

        vector.sort();

        vector
            .iter()
            .fold(Array::with_capacity(length), |mut acc, e| {
                acc.push(Fixnum::new(*e))
            })
    }

    fn par_sort(input: Array) -> Array {
        let input = input.unwrap();
        let length = input.length();

        let mut vector = (0..length).fold(Vec::with_capacity(length), |mut acc, i| {
            let number = unsafe { input.at(i as i64).to::<Fixnum>().to_i64() };

            acc.push(number);

            acc
        });

        vector.par_sort();

        vector
            .iter()
            .fold(Array::with_capacity(length), |mut acc, e| {
                acc.push(Fixnum::new(*e))
            })
    }
}

#[no_mangle]
pub fn initialize_parallel_sort() {
    Class::from_existing("ParallelSort")
        .define_nested_class("Integers", None)
        .define(|itself| {
            itself.def_self("sort", sort);
            itself.def_self("par_sort", par_sort);
        });

    println!("Hello, World!");
}
