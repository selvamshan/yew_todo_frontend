use stylist::{css, yew::styled_component};
use yew::prelude::*;

use crate::{components::automs::ss_links::SSLink, rounter::Route};


#[styled_component(NavBar)]
pub fn navbar() -> Html {
    let stylesheet = css!(
        r#"
            border-bottom: 1px solid antiquewhite;
            padding: 10px 20px;
        "#
    )    ;
    html!{
        <section class={stylesheet}>
            <SSLink  text="Todo" data_test="logo" route={Route::Home}/>
        </section>
    }
}