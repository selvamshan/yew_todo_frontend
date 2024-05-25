use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::api::AuthResponse;



#[derive(Clone, Debug, Serialize, Deserialize, Store, PartialEq)]
#[store(storage="local")]
pub struct Store{
    pub username: String,
    pub token : String
}


impl Default for Store {
    fn default() -> Self {
        Self {
            username: Default::default(),
            token: Default::default(),
        }
    }
}


pub fn login_reducer(auth_response: AuthResponse, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.username = auth_response.data.username;
        store.token = auth_response.data.token;
    });
}

pub fn logout(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(|store| {
        store.username = String::new();
        store.token = String::new();
        //store.tasks = vec![];
    });
}