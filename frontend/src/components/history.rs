use chrono::{Datelike, Local};
use common::AttendanceEntry;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HistoryProps {
    pub entries: Vec<AttendanceEntry>,
}

#[function_component(HistoryView)]
pub fn history_view(props: &HistoryProps) -> Html {
    let now = Local::now();

    html! {
        <section class="page active">
            <div class="topbar">
                <div>
                    <div class="greeting">{ "Previous records" }</div>
                    <div class="page-title">{ "History" }</div>
                </div>
            </div>

            <div class="history-list">
                if props.entries.is_empty() {
                    <div class="history-card neu">
                        <div class="history-date">
                            <div class="history-day neu-small">
                                <strong>{ now.day() }</strong>
                                <span>{ now.format("%b").to_string().to_uppercase() }</span>
                            </div>
                            <div class="history-details">
                                <strong>{ "No recent attendance" }</strong>
                                <span>{ "Start marking meals to see history." }</span>
                            </div>
                        </div>
                    </div>
                } else {
                    { for props.entries.iter().map(|entry| {
                        html! {
                            <div class="history-card neu">
                                <div class="history-date">
                                    <div class="history-day neu-small">
                                        <strong>{ entry.date.day() }</strong>
                                        <span>{ entry.date.format("%b").to_string().to_uppercase() }</span>
                                    </div>
                                    <div class="history-details">
                                        <strong>{ format!("{:?} attended", entry.meal) }</strong>
                                        <span>{ format!("Status: {}", if entry.response { "Present" } else { "Absent" }) }</span>
                                    </div>
                                </div>
                                <div class="history-badge">{ "Completed" }</div>
                            </div>
                        }
                    }) }
                }
            </div>
        </section>
    }
}
