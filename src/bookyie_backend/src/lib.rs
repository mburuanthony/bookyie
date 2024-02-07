#![allow(unused)]
#[macro_use]
extern crate serde;
use books::Book;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use reviews::Review;
use std::{borrow::Cow, cell::RefCell};

mod books;
mod reviews;
mod users;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;
type BooksData = (u64, books::Book);
type UserReview = (u64, users::User, reviews::Review);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("failed to create counter")
    );

    static BOOKSSTORE: RefCell<StableBTreeMap<u64, books::Book, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static REVIEWSSTORE: RefCell<StableBTreeMap<u64, reviews::Review, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static USERSSTORE: RefCell<StableBTreeMap<u64, users::User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

#[ic_cdk::update]
fn create_user(user: users::User) -> Option<users::User> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("failed to update counter");
    let user = users::User {
        id,
        date_joined: time(),
    };
    _add_user(user.clone());
    Some(user)
}

#[ic_cdk::update]
fn delete_user(id: u64) -> Result<users::User, Error> {
    match USERSSTORE.with(|service| service.borrow_mut().remove(&id)) {
        Some(user) => Ok(user),
        None => Err(Error::NotFound {
            msg: format!("failed to delete user"),
        }),
    }
}

#[ic_cdk::query]
fn get_books() -> Result<Vec<BooksData>, Error> {
    match _fetch_books() {
        Some(books) => Ok(books),
        None => Err(Error::NotFound {
            msg: format!("temporarily unable to load books..."),
        }),
    }
}

#[ic_cdk::update]
fn delete_review(id: u64) -> Result<reviews::Review, Error> {
    match REVIEWSSTORE.with(|service| service.borrow_mut().remove(&id)) {
        Some(user) => Ok(user),
        None => Err(Error::NotFound {
            msg: format!("failed to delete review"),
        }),
    }
}

fn _add_user(user: users::User) {
    USERSSTORE.with(|service| service.borrow_mut().insert(user.id, user.clone()));
}

fn _fetch_user(id: &u64) -> Option<users::User> {
    USERSSTORE.with(|s| s.borrow().get(id))
}

fn _fetch_books() -> Option<Vec<BooksData>> {
    Some(BOOKSSTORE.with(|s| s.borrow().iter().collect()))
}

fn _fetch_review(id: &u64) -> Option<reviews::Review> {
    REVIEWSSTORE.with(|s| s.borrow().get(id))
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

ic_cdk::export_candid!();
