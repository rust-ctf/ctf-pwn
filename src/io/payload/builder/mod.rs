mod build;
mod dynamic_payload;
mod print;
mod read;
mod send;
mod arch;
mod convert;
mod condition;

use crate::io::payload::payloads;
use crate::io::payload::payloads::{
    Ascii, Building, Chain, Complete, DynamicPayload, Initial, ReadPayload, SendPayload,
};
use crate::io::*;
use crossterm::Command;
use std::marker::PhantomData;

pub struct PayloadBuilder<T, A> {
    payload: T,
    _phantom_arch: PhantomData<A>,
}

impl<A> PayloadBuilder<Initial, A> {
    pub fn new() -> PayloadBuilder<Initial, A> {
        PayloadBuilder::<Initial, A> {
            payload: Initial {},
            _phantom_arch: PhantomData::default(),
        }
    }
}

impl<T: PayloadAction, A> PayloadBuilder<T, A> {
    pub fn from(payload: T) -> PayloadBuilder<T, A> {
        PayloadBuilder::<T, A> {
            payload,
            _phantom_arch: PhantomData::default(),
        }
    }
}

pub trait Buildable: PayloadAction {}
pub trait Sendable: PayloadAction {
    fn push<A, T: AsRef<[u8]>>(self, data: T) -> impl SendCompletable
    where
        Self: Sized,
    {
        Chain::new(self, SendPayload::<Building, A>::new().push::<A, T>(data))
    }
}
pub trait SendCompletable: Sendable {
    fn complete(self) -> impl Buildable + Readable + Sendable;
}

pub trait Readable: PayloadAction {}
pub trait ReturnsValue: PayloadAction {}
