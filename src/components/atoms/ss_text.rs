use stylist::{css, yew::styled_component, Style};
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TextType {
    Normal,
    Title,
}

impl Default for TextType {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Normal,
    Danger,
    Info,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Normal => "normal",
            Color::Danger => "danger",
            Color::Info => "info",
        }
        .to_owned()
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
    pub data_test: String,
    #[prop_or_default]
    pub text_type: Option<TextType>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub color: Option<Color>,
}

#[styled_component(SSText)]
pub fn ss_text(props: &Props) -> Html {
    let text_type = props.text_type.unwrap_or_default();
    let color = props.color.unwrap_or_default();
    let stylesheet = Style::new(css!(
        r#"
            .danger {
                color: red;
            }

            .normal {
                color: floralwhite;
            }

            .info {
                color: lightblue;
            }
        "#
    ))
    .unwrap();

    html! {
        <span class={stylesheet}>
            {
                match text_type {
                    TextType::Normal => normal_text(
                        props.data_test.clone(),
                        &props.text,
                        vec![props.class.clone().unwrap_or_default(), color.to_string()],
                    ),
                    TextType::Title => title_text(
                        props.data_test.clone(),
                        &props.text,
                        vec![props.class.clone().unwrap_or_default(), color.to_string()],
                    ),
                }
            }
        </span>
    }
}

fn normal_text(data_test: String, text: &str, class: Vec<String>) -> Html {
    let stylesheet = Style::new(css!(
        r#"
            font-size: 36px;
        "#
    ))
    .unwrap();

    html! {
      <p class={classes!(stylesheet, class)} data-test={data_test}>{text}</p>
    }
}

fn title_text(data_test: String, text: &str, class: Vec<String>) -> Html {
    let stylesheet = Style::new(css!(
        r#"
      font-size: 72px;
    "#
    ))
    .unwrap();

    html! {
      <h1 data-test={data_test} class={classes!(stylesheet, class)}>{text}</h1>
    }
}