#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;

use typefamilies::*;
use typefamilies_derive::*;

#[derive(Family)]
struct SingleParam<T>(PhantomData<T>);


#[derive(Family)]
struct DoubleParam<T, U>(PhantomData<(T, U)>);

#[derive(Family)]
struct TripleParam<T, U, V>(PhantomData<(T, U, V)>);
