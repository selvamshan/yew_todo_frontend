use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::api::{AuthResponse, TaskResponse};



#[derive(Clone, Debug, Serialize, Deserialize, Store, PartialEq)]
#[store(storage="local")]
pub struct Store{
    pub username: String,
    pub token : String,
    pub tasks: Vec<Task>,    
}


impl Default for Store {
    fn default() -> Self {
        Self {
            username: Default::default(),
            token: Default::default(),
            tasks: Default::default(),            
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Task {
    pub completed_at: Option<String>,
    pub description: Option<String>,
    pub id: u32,
    pub priority: Option<char>,
    pub title: String,
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

pub fn set_tasks(tasks: TaskResponse, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.tasks = tasks.data;
    })
}

pub fn add_task(dispatch: Dispatch<Store>, task: Task) {
    dispatch.reduce_mut(move |store| {
        store.tasks.push(task);
    });
}

