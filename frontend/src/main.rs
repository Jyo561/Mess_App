mod api;
mod components;
mod storage;
mod types;

use chrono::Local;
use common::{AttendanceEntry, LoginResponse, MealType};
use gloo_timers::callback::Timeout;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::{
    attendance::AttendanceView,
    concession::ConcessionView,
    history::HistoryView,
    home::HomeView,
    login::LoginView,
    meal_modal::MealModal,
    navbar::Navbar,
    profile::ProfileView,
    toast::Toast,
};
use crate::types::Page;

#[function_component(App)]
fn app() -> Html {
    let session = use_state(storage::get_session);
    let current_page = use_state(|| Page::Home);
    let today_entries = use_state(|| Vec::<AttendanceEntry>::new());
    let history_entries = use_state(|| Vec::<AttendanceEntry>::new());
    let modal_meal = use_state(|| Option::<MealType>::None);
    let toast_msg = use_state(|| Option::<String>::None);

    let show_toast = {
        let toast_msg = toast_msg.clone();
        Callback::from(move |msg: String| {
            toast_msg.set(Some(msg));
            let toast_msg = toast_msg.clone();
            Timeout::new(2500, move || toast_msg.set(None)).forget();
        })
    };

    // Load attendance records upon login
    {
        let today_entries = today_entries.clone();
        let history_entries = history_entries.clone();
        let is_logged_in = session.is_some();
        use_effect_with(is_logged_in, move |logged_in| {
            if *logged_in {
                let today_entries = today_entries.clone();
                let history_entries = history_entries.clone();
                spawn_local(async move {
                    if let Ok(records) = api::get_history().await {
                        let today = Local::now().date_naive();
                        let filtered = records.iter().filter(|e| e.date == today).cloned().collect();
                        today_entries.set(filtered);
                        history_entries.set(records);
                    }
                });
            }
        });
    }

    let on_login_success = {
        let session = session.clone();
        Callback::from(move |res: LoginResponse| {
            storage::save_session(&res);
            session.set(Some(res));
        })
    };

    let on_logout = {
        let session = session.clone();
        let current_page = current_page.clone();
        let show_toast = show_toast.clone();
        Callback::from(move |_| {
            storage::clear_session();
            session.set(None);
            current_page.set(Page::Home);
            show_toast.emit("Logged out".into());
        })
    };

    let on_confirm_meal = {
        let modal_meal = modal_meal.clone();
        let session = session.clone();
        let today_entries = today_entries.clone();
        let show_toast = show_toast.clone();

        Callback::from(move |meal: MealType| {
            if let Some(ref user) = *session {
                let entry = AttendanceEntry {
                    id: None,
                    user_id: user.user_id.clone(),
                    meal,
                    date: Local::now().date_naive(),
                    response: true,
                    remarks: "Attending".into(),
                };

                let modal_meal = modal_meal.clone();
                let today_entries = today_entries.clone();
                let show_toast = show_toast.clone();

                spawn_local(async move {
                    match api::submit_entry(entry.clone()).await {
                        Ok(new_entry) => {
                            let mut current = (*today_entries).clone();
                            current.retain(|e| e.meal != meal);
                            current.push(new_entry);
                            today_entries.set(current);
                            modal_meal.set(None);
                            show_toast.emit(format!("{:?} marked successfully", meal));
                        }
                        Err(e) => {
                            show_toast.emit(format!("Failed: {e}"));
                        }
                    }
                });
            }
        })
    };

    html! {
        <div>
            if let Some(ref user) = *session {
                <div class="app">
                    <Navbar
                        current_page={*current_page}
                        on_navigate={{
                            let current_page = current_page.clone();
                            Callback::from(move |p| current_page.set(p))
                        }}
                    />

                    <main class="main">
                        <div class="content">
                            if *current_page == Page::Home {
                                <HomeView
                                    user={user.clone()}
                                    today_entries={(*today_entries).clone()}
                                    on_open_modal={{
                                        let modal_meal = modal_meal.clone();
                                        Callback::from(move |m| modal_meal.set(Some(m)))
                                    }}
                                    on_navigate={{
                                        let current_page = current_page.clone();
                                        Callback::from(move |p| current_page.set(p))
                                    }}
                                />
                            }
                            if *current_page == Page::Attendance {
                                <AttendanceView today_entries={(*today_entries).clone()} />
                            }
                            if *current_page == Page::History {
                                <HistoryView entries={(*history_entries).clone()} />
                            }
                            if *current_page == Page::Concession {
                                <ConcessionView show_toast={show_toast.clone()} />
                            }
                            if *current_page == Page::Profile {
                                <ProfileView
                                    user={user.clone()}
                                    on_logout={on_logout.clone()}
                                    on_navigate={{
                                        let current_page = current_page.clone();
                                        Callback::from(move |p| current_page.set(p))
                                    }}
                                />
                            }
                        </div>
                    </main>

                    <MealModal
                        meal={*modal_meal}
                        on_confirm={on_confirm_meal}
                        on_cancel={{
                            let modal_meal = modal_meal.clone();
                            Callback::from(move |_| modal_meal.set(None))
                        }}
                    />
                </div>
            } else {
                <LoginView
                    on_login_success={on_login_success}
                    show_toast={show_toast.clone()}
                />
            }

            <Toast message={(*toast_msg).clone()} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
