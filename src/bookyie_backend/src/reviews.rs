#![allow(unused)]
#[macro_use]
use std::borrow::Cow;
use crate::{books::Book, users::User};
use candid::{Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Review {
    pub id: u64,
    pub review_message: String,
    pub review_date: String,
    pub book_reviewd: Book,
    pub reviewd_by: User,
}

impl Storable for Review {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Review {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}
