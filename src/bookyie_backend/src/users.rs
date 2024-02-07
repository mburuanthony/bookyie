#![allow(unused)]
#[macro_use]
use std::borrow::Cow;
use crate::books::Book;
use candid::{Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct User {
    pub id: u64,
    pub date_joined: u64,
}

impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}
