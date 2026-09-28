use common::LoginResponse;
use crate::types::Page;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProfileProps {
    pub user: LoginResponse,
    pub on_logout: Callback<()>,
    pub on_navigate: Callback<Page>,
}

#[function_component(ProfileView)]
pub fn profile_view(props: &ProfileProps) -> Html {
    let user_initials: String = props.user.name
        .split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect();

    let on_logout_click = {
        let on_logout = props.on_logout.clone();
        Callback::from(move |_| on_logout.emit(()))
    };

    html! {
        <section class="page active">
            <div class="topbar">
                <div>
                    <div class="greeting">{ "Your account" }</div>
                    <div class="page-title">{ "Profile" }</div>
                </div>
            </div>

            <div class="profile-header neu">
                <div class="profile-avatar neu-small">{ user_initials }</div>
                <h2>{ &props.user.name }</h2>
                <p>{ format!("IC: {} | {} ({})", props.user.user_id, props.user.designation, props.user.ci) }</p>
                <div class="verified">{ "✓ Verified Account" }</div>
            </div>

            <div class="profile-options">
                <div class="profile-option neu" onclick={{
                    let nav = props.on_navigate.clone();
                    Callback::from(move |_| nav.emit(Page::Attendance))
                }}>
                    <div class="profile-option-left">
                        <div class="profile-option-icon neu-small">{ "📅" }</div>
                        <strong>{ "Attendance Summary" }</strong>
                    </div>
                    <span class="arrow">{ "→" }</span>
                </div>

                <div class="profile-option neu" onclick={{
                    let nav = props.on_navigate.clone();
                    Callback::from(move |_| nav.emit(Page::Concession))
                }}>
                    <div class="profile-option-left">
                        <div class="profile-option-icon neu-small">{ "📄" }</div>
                        <strong>{ "My Concessions" }</strong>
                    </div>
                    <span class="arrow">{ "→" }</span>
                </div>

                <div class="profile-option neu logout" onclick={on_logout_click}>
                    <div class="profile-option-left">
                        <div class="profile-option-icon neu-small">{ "↪" }</div>
                        <strong>{ "Logout" }</strong>
                    </div>
                </div>
            </div>
        </section>
    }
}
