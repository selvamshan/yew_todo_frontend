use std::ops::Deref;

use stylist::{yew::styled_component, Style};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::api::get_tasks;
use crate::store::{set_tasks, Store};
use crate::components::organisms::tasks::Tasks;

#[styled_component(Home)] 
pub fn home() -> Html {
    let stylesheet = Style::new(css!(
        r#"
            display: flex;
            flex-direction: column;
            aling-itesm:center;
         "#
    )).unwrap();

    let (store, dispatch) = use_store::<Store>();  
    let tasks = store.deref().tasks.clone();

    use_effect( move || {
        let token = store.deref().token.clone();
        if !token.is_empty() {     
            spawn_local(async move  {
                let tasks = get_tasks(&token).await;
                set_tasks(tasks, dispatch.clone());                
            });
        }

        || {}       
    });

    html!{
        <section class={stylesheet}>       
        <Tasks tasks={tasks} />
        </section>
    }
}