use stylist::yew::styled_component;
use yew::prelude::*;
//use yewdux::prelude::*;



#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: u32,
}


#[styled_component(OneTask)] 
pub fn one_task(props: &Props) -> Html {
    
    html!{

    }
}