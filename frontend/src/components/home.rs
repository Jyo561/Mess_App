use chrono::{Datelike, Local, Timelike};
use common::{AttendanceEntry, LoginResponse, MealType};
use crate::types::Page;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub user: LoginResponse,
    pub today_entries: Vec<AttendanceEntry>,
    pub on_open_modal: Callback<MealType>,
    pub on_navigate: Callback<Page>,
}

#[function_component(HomeView)]
pub fn home_view(props: &HomeProps) -> Html {
    let now = Local::now();
    let hour = now.hour();
    let greeting = if hour < 12 { "Good morning," } else if hour < 17 { "Good afternoon," } else { "Good evening," };
    let today_date_str = now.format("%A, %-d %B").to_string();

    let user_initials: String = props.user.name
        .split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect();

    let is_meal_marked = |m: MealType| {
        props.today_entries.iter().any(|e| e.meal == m && e.response)
    };

    let render_meal = |meal: MealType, title: &'static str, icon: &'static str, class_name: &'static str| {
        let marked = is_meal_marked(meal);
        let on_mark = {
            let on_open = props.on_open_modal.clone();
            Callback::from(move |_| on_open.emit(meal))
        };

        html! {
            <div class="meal-card neu">
                <div class="meal-info">
                    <div class={format!("meal-icon neu-small {}", class_name)}>{ icon }</div>
                    <div>
                        <div class="meal-name">{ title }</div>
                        <div class="meal-status">{ if marked { "Attending" } else { "Not marked" } }</div>
                    </div>
                </div>
                if marked {
                    <div class="marked">{ "✓ Marked" }</div>
                } else {
                    <button class="meal-btn" onclick={on_mark}>{ "Mark" }</button>
                }
            </div>
        }
    };

    html! {
        <section class="page active">
            <div class="topbar">
                <div>
                    <div class="greeting">{ greeting }</div>
                    <div class="page-title">{ format!("{} 👋", props.user.name) }</div>
                </div>
                <div class="avatar neu-small" onclick={{
                    let nav = props.on_navigate.clone();
                    Callback::from(move |_| nav.emit(Page::Profile))
                }}>
                    { user_initials }
                </div>
            </div>

            <div class="date-card neu-small">
                <div class="date-left">
                    <div class="date-icon neu-small">{ "📅" }</div>
                    <div class="date-info">
                        <strong>{ today_date_str }</strong>
                        <span>{ "Today's meals" }</span>
                    </div>
                </div>
                <span>{ now.year().to_string() }</span>
            </div>

            <h2 class="section-title">{ "Today's Meals" }</h2>
            <div class="meals">
                { render_meal(MealType::Breakfast, "Breakfast", "🍳", "breakfast") }
                { render_meal(MealType::Lunch, "Lunch", "🍛", "lunch") }
                { render_meal(MealType::Dinner, "Dinner", "🍽", "dinner") }
            </div>

            // Concession Quick Card
            <div class="concession-card neu" onclick={{
                let nav = props.on_navigate.clone();
                Callback::from(move |_| nav.emit(Page::Concession))
            }}>
                <div class="concession-left">
                    <div class="concession-icon neu-small">{ "📄" }</div>
                    <div class="concession-text">
                        <strong>{ "Going away?" }</strong>
                        <span>{ "Apply for Mess Concession" }</span>
                    </div>
                </div>
                <div class="arrow">{ "→" }</div>
            </div>

            // Quick Links
            <h2 class="section-title" style="margin-top:28px;">{ "Quick Links" }</h2>
            <div class="quick-links">
                <div class="quick-card neu" onclick={{
                    let nav = props.on_navigate.clone();
                    Callback::from(move |_| nav.emit(Page::Attendance))
                }}>
                    <div class="quick-card-icon">{ "📅" }</div>
                    <strong>{ "Attendance" }</strong>
                    <span>{ "View attendance" }</span>
                </div>
                <div class="quick-card neu" onclick={{
                    let nav = props.on_navigate.clone();
                    Callback::from(move |_| nav.emit(Page::History))
                }}>
                    <div class="quick-card-icon">{ "🕘" }</div>
                    <strong>{ "History" }</strong>
                    <span>{ "Previous records" }</span>
                </div>
            </div>
        </section>
    }
}
