use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ToastProps {
    pub message: Option<String>,
}

#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let show_class = if props.message.is_some() { "toast show" } else { "toast" };
    html! {
        <div class={show_class}>
            { props.message.as_deref().unwrap_or_default() }
        </div>
    }
}
