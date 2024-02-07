#![allow(unused)]
#[macro_use]
use std::borrow::Cow;
use candid::{Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Book {
    pub id: u64,
    pub book_title: String,
    pub book_description: String,
    pub author_name: String,
    pub publisher: String,
    pub year_published: i32,
}

impl Storable for Book {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Book {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}
