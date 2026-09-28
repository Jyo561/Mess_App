use chrono::{Datelike, Local};
use common::{AttendanceEntry, MealType};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AttendanceProps {
    pub today_entries: Vec<AttendanceEntry>,
}

#[function_component(AttendanceView)]
pub fn attendance_view(props: &AttendanceProps) -> Html {
    let now = Local::now();
    let today_day = now.day();
    let month_year = now.format("%B %Y").to_string();

    let is_marked = |m: MealType| {
        props.today_entries.iter().any(|e| e.meal == m && e.response)
    };

    html! {
        <section class="page active">
            <div class="topbar">
                <div>
                    <div class="greeting">{ "Your records" }</div>
                    <div class="page-title">{ "Attendance" }</div>
                </div>
            </div>

            // Interactive Month Calendar Card
            <div class="calendar-card neu">
                <div class="calendar-header">
                    <button class="calendar-arrow neu-small">{ "‹" }</button>
                    <strong>{ month_year }</strong>
                    <button class="calendar-arrow neu-small">{ "›" }</button>
                </div>

                <div class="weekdays">
                    <div class="weekday">{ "Mon" }</div>
                    <div class="weekday">{ "Tue" }</div>
                    <div class="weekday">{ "Wed" }</div>
                    <div class="weekday">{ "Thu" }</div>
                    <div class="weekday">{ "Fri" }</div>
                    <div class="weekday">{ "Sat" }</div>
                    <div class="weekday">{ "Sun" }</div>
                </div>

                <div class="calendar-grid">
                    <div class="calendar-day empty"></div>
                    { for (1..=30).map(|d| {
                        let class_name = if d == today_day { "calendar-day today" } else { "calendar-day" };
                        html! { <div class={class_name}>{ d }</div> }
                    }) }
                </div>
            </div>

            // Daily Meal Status Breakdown
            <div class="daily-card neu">
                <h2 class="section-title">{ format!("Daily Records — {}", now.format("%-d %B %Y")) }</h2>

                <div class="daily-row">
                    <div class="daily-row-left">
                        <div class={if is_marked(MealType::Breakfast) { "status-dot success" } else { "status-dot muted" }}></div>
                        <span class="status-text">{ "🍳 Breakfast" }</span>
                    </div>
                    <span class={if is_marked(MealType::Breakfast) { "marked" } else { "" }}>
                        { if is_marked(MealType::Breakfast) { "Attending" } else { "Not marked" } }
                    </span>
                </div>

                <div class="daily-row">
                    <div class="daily-row-left">
                        <div class={if is_marked(MealType::Lunch) { "status-dot success" } else { "status-dot muted" }}></div>
                        <span class="status-text">{ "🍛 Lunch" }</span>
                    </div>
                    <span class={if is_marked(MealType::Lunch) { "marked" } else { "" }}>
                        { if is_marked(MealType::Lunch) { "Attending" } else { "Not marked" } }
                    </span>
                </div>

                <div class="daily-row">
                    <div class="daily-row-left">
                        <div class={if is_marked(MealType::Dinner) { "status-dot success" } else { "status-dot muted" }}></div>
                        <span class="status-text">{ "🍽 Dinner" }</span>
                    </div>
                    <span class={if is_marked(MealType::Dinner) { "marked" } else { "" }}>
                        { if is_marked(MealType::Dinner) { "Attending" } else { "Not marked" } }
                    </span>
                </div>
            </div>

            // Summary Card
            <div class="summary-card neu">
                <h2 class="section-title">{ "Monthly Summary" }</h2>
                <div class="summary-grid">
                    <div class="summary-item neu-inset">
                        <strong>{ "21" }</strong>
                        <span>{ "Breakfast" }</span>
                    </div>
                    <div class="summary-item neu-inset">
                        <strong>{ "23" }</strong>
                        <span>{ "Lunch" }</span>
                    </div>
                    <div class="summary-item neu-inset">
                        <strong>{ "19" }</strong>
                        <span>{ "Dinner" }</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
