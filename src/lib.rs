use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod rounter;
mod components;
use rounter::{Route,switch};
use components::organisms::navbar::NavBar;


#[function_component(App)]
pub fn app() -> Html {
    html!{           
        <BrowserRouter>   
            <NavBar />   
            <Switch<Route> render={Switch::render(switch)} />     
        </BrowserRouter>        
    }
}
