mod build;
mod dynamic_payload;
mod print;
mod read;
mod send;

use crate::io::payload::payloads;
use crate::io::payload::payloads::{
    Ascii, Building, Chain, Complete, DynamicPayload, Initial, ReadPayload, SendPayload,
};
use crate::io::PayloadAction;
use crossterm::Command;
use std::marker::PhantomData;

pub struct PayloadBuilder<T, A> {
    payload: T,
    _phantom_arch: PhantomData<A>,
}

impl<A> PayloadBuilder<Initial, A> {
    pub(crate) fn new() -> PayloadBuilder<Initial, A> {
        PayloadBuilder::<Initial, A> {
            payload: Initial {},
            _phantom_arch: PhantomData::default(),
        }
    }
}

impl<T, A> PayloadBuilder<T, A> {
    pub(crate) fn from(payload: T) -> PayloadBuilder<T, A> {
        PayloadBuilder::<T, A> {
            payload,
            _phantom_arch: PhantomData::default(),
        }
    }
}

pub(crate) trait Buildable: PayloadAction {}
pub(crate) trait Sendable: PayloadAction {
    fn push<A, T: AsRef<[u8]>>(self, data: T) -> impl PayloadAction
    where
        Self: Sized,
    {
        Chain::new(self, SendPayload::<Building, A>::new().push::<A, T>(data))
    }
}
pub(crate) trait Readable: PayloadAction {}
pub(crate) trait ReturnsValue: PayloadAction {}
