use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

mod pages;
mod router;
mod store;
mod components;
mod api;
use router::{Route, switch};
use store::Store;
use components::organisms::navbar::Navbar;


#[function_component(App)]
pub fn app() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let token = store.token.clone();
    let is_loaded = use_state(|| false);
    html!{           
        <BrowserRouter>   
            <Navbar />   
            <Switch<Route> render={switch} />     
        </BrowserRouter>        
    }
}
