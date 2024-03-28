#![allow(dead_code)]

use typefamilies::*;

#[derive(Family)]
struct SingleParam<T>(T);

#[derive(Family)]
struct DoubleParam<T, U>(T, U);
