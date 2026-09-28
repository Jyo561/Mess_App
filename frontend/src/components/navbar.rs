use crate::types::Page;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub current_page: Page,
    pub on_navigate: Callback<Page>,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let nav_items = [
        (Page::Home, "🏠", "Home"),
        (Page::Attendance, "📅", "Attendance"),
        (Page::History, "🕘", "History"),
        (Page::Concession, "📄", "Concession"),
        (Page::Profile, "👤", "Profile"),
    ];

    html! {
        <>
            // Desktop Sidebar
            <aside class="sidebar neu">
                <div class="brand">
                    <div class="brand-icon neu-small">{ "🏠" }</div>
                    <div class="brand-text">
                        <strong>{ "Enclave" }</strong>
                        <span>{ "Mess Management" }</span>
                    </div>
                </div>

                <nav class="sidebar-nav">
                    { for nav_items.iter().map(|(page, icon, label)| {
                        let active = if *page == props.current_page { "nav-item active" } else { "nav-item" };
                        let on_click = {
                            let on_navigate = props.on_navigate.clone();
                            let page = *page;
                            Callback::from(move |_| on_navigate.emit(page))
                        };
                        html! {
                            <button class={active} onclick={on_click}>
                                { format!("{} {}", icon, label) }
                            </button>
                        }
                    }) }
                </nav>
            </aside>

            // Mobile Bottom Navigation Bar
            <nav class="bottom-nav neu">
                { for nav_items.iter().map(|(page, icon, label)| {
                    let active = if *page == props.current_page { "bottom-nav-item active" } else { "bottom-nav-item" };
                    let on_click = {
                        let on_navigate = props.on_navigate.clone();
                        let page = *page;
                        Callback::from(move |_| on_navigate.emit(page))
                    };
                    html! {
                        <button class={active} onclick={on_click}>
                            { icon }
                            <span>{ label }</span>
                        </button>
                    }
                }) }
            </nav>
        </>
    }
}
