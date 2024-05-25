use std::ops::Deref;

use crate::components::atoms::ss_button::{SSButton, ButtonColor};
use crate::components::atoms::ss_link::{SSLink, LinkType};
use crate::components::atoms::ss_text::SSText;

use crate::router::Route;
use crate::store::Store;
use crate::{api, store};
use stylist::Style;
use stylist::{css, yew::styled_component};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let stylesheet = Style::new(css!(
        r#"
          section {
            border-bottom: 1px solid antiquewhite;
            padding: 10px 20px;
            display: flex;
            justify-content: space-between;
          }

          .nav-right {
            display: flex;
          }

          .nav-right button {
            margin-left: 10px;
          }
        "#
    ))
    .unwrap();

    let (store, dispatch) = use_store::<Store>();
    let username = store.username.clone();
    let token = store.token.clone();

    let logout_onclick = {
        let token = store.token.clone();
        let history = use_navigator().unwrap();
        Callback::from(move |_event: MouseEvent| {
            let token = token.clone();
            let dispatch = dispatch.clone();
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::logout(&token).await {
                    Ok(_) => {
                        store::logout(dispatch);
                        history.push(&Route::Home);
                    }
                    Err(error) => gloo::console::error!(error.to_string()),
                }
            })
        })
    };

    html! {
      <div class={stylesheet}>
      <section>
        <SSLink text={"Todo".to_owned()} data_test={"logo".to_owned()} route={Route::Home} />
        // if is_logged_in(&token) {
        //   <div>
        //     <TaskEditButtons />
        //   </div>
        // }
        if !is_logged_in(&token) {
          <div>
            <SSLink text={"Create Account".to_owned()} data_test={"create-account".to_owned()} route={Route::CreateAccount} link_type={LinkType::Button} />
            <SSLink text={"Login".to_owned()} data_test={"login".to_owned()} route={Route::Login} link_type={LinkType::Button} />
            </div>
          } else {
            <div class="nav-right">
              <SSText data_test="welcome" text={format!("Welcome, {}", username)} /> // 
              <SSLink text={"Logout".to_owned()} data_test={"logout".to_owned()} route={Route::Home} link_type={LinkType::Button} />
              <SSButton
                data_test="logout"
                label="Logout"
                onclick={logout_onclick}
                color={ButtonColor::Normal}
              />
            </div>
        }
      </section>
      </div>
    }
}


fn is_logged_in(token: &str) -> bool {
    !token.deref().is_empty()
}