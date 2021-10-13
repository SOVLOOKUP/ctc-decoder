#![feature(static_nobundle)]

#[macro_use(s)]
#[cfg_attr(test, macro_use(array))]
extern crate ndarray;
mod duplex;
mod fastexp;
mod search;
mod tree;
mod vec2d;

#[derive(Clone, Copy, Debug)]
pub enum SearchError {
    RanOutOfBeam,
    IncomparableValues,
    InvalidEnvelope,
}
