use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{create_account::CreateAccount, home::Home, login::Login, one_task::OneTask};


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/create-account")]
    CreateAccount,

    #[at("/login")]
    Login,

    #[at("/tasks/:id")]
    OneTask { id: u32 },
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <Home />},
        Route::CreateAccount => html!{ <CreateAccount /> },
        Route::Login => html! { <Login /> },
        Route::OneTask { id } => html!{ <OneTask id={id} />}
    }
}