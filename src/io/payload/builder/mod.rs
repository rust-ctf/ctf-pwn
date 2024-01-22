mod send;
mod read;

use std::marker::PhantomData;
use crossterm::Command;
use crate::io::PayloadAction;
use crate::io::payload::payloads;
use crate::io::payload::payloads::{Ascii, Building, Chain, Complete, DynamicPayload, Initial, ReadPayload, SendPayload};

pub struct PayloadBuilder<T, A>
{
    payload: T,
    _phantom_arch: PhantomData<A>,
}

impl<A> PayloadBuilder<Initial,A>
{
    pub(crate) fn new() -> PayloadBuilder<Initial, A>
    {
        PayloadBuilder::<Initial, A> { payload: Initial { }, _phantom_arch: PhantomData::default() }
    }
}


impl<T,A> PayloadBuilder<T,A>
{
    pub(crate) fn from(payload: T) -> PayloadBuilder<T,A>
    {
        PayloadBuilder::<T, A> { payload, _phantom_arch: PhantomData::default()  }
    }
}


impl<A> PayloadBuilder<Initial, A>
{

}

impl<P1: PayloadAction, E, A> PayloadBuilder<Chain<P1,ReadPayload<E>>, A>
{

}

impl<P1: PayloadAction, A> PayloadBuilder<Chain<P1,SendPayload<Complete, A>>,A>
{

}

impl<P, E, T, A> PayloadBuilder<DynamicPayload<P,E,T>, A>
{

}
