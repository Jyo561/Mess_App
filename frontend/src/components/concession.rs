use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum ConcessionTab {
    NewRequest,
    MyRequests,
}

#[derive(Properties, PartialEq)]
pub struct ConcessionProps {
    pub show_toast: Callback<String>,
}

#[function_component(ConcessionView)]
pub fn concession_view(props: &ConcessionProps) -> Html {
    let active_tab = use_state(|| ConcessionTab::NewRequest);

    let from_date = use_state(String::new);
    let to_date = use_state(String::new);
    let reason = use_state(String::new);

    let on_submit = {
        let show_toast = props.show_toast.clone();
        let active_tab = active_tab.clone();
        Callback::from(move |_| {
            show_toast.emit("Concession request submitted".into());
            active_tab.set(ConcessionTab::MyRequests);
        })
    };

    html! {
        <section class="page active">
            <div class="topbar">
                <div>
                    <div class="greeting">{ "Manage your requests" }</div>
                    <div class="page-title">{ "Concession" }</div>
                </div>
            </div>

            <div class="tabs neu-inset">
                <button
                    class={if *active_tab == ConcessionTab::NewRequest { "tab active" } else { "tab" }}
                    onclick={{
                        let active_tab = active_tab.clone();
                        Callback::from(move |_| active_tab.set(ConcessionTab::NewRequest))
                    }}
                >
                    { "New Request" }
                </button>
                <button
                    class={if *active_tab == ConcessionTab::MyRequests { "tab active" } else { "tab" }}
                    onclick={{
                        let active_tab = active_tab.clone();
                        Callback::from(move |_| active_tab.set(ConcessionTab::MyRequests))
                    }}
                >
                    { "My Requests" }
                </button>
            </div>

            if *active_tab == ConcessionTab::NewRequest {
                <div class="form-card neu">
                    <div class="form-field">
                        <label>{ "FROM DATE" }</label>
                        <input class="form-input neu-inset" type="date" />
                    </div>

                    <div class="form-field">
                        <label>{ "TO DATE" }</label>
                        <input class="form-input neu-inset" type="date" />
                    </div>

                    <div class="form-field">
                        <label>{ "REASON" }</label>
                        <textarea class="form-textarea neu-inset" placeholder="Reason for concession / Leave order..."></textarea>
                    </div>

                    <div class="info-box neu-inset">
                        { "ℹ️ Your mess billing is adjusted for meals during approved concession days." }
                    </div>

                    <button class="primary-btn" onclick={on_submit}>
                        { "Submit Request" }
                    </button>
                </div>
            } else {
                <div class="request-list">
                    <div class="request-card neu">
                        <div class="request-top">
                            <div>
                                <div class="request-date">{ "01 Oct — 05 Oct 2026" }</div>
                                <div class="request-reason">{ "Going home for family function" }</div>
                            </div>
                            <div class="request-status pending">{ "Pending" }</div>
                        </div>
                    </div>

                    <div class="request-card neu">
                        <div class="request-top">
                            <div>
                                <div class="request-date">{ "12 Aug — 14 Aug 2026" }</div>
                                <div class="request-reason">{ "Command seminar" }</div>
                            </div>
                            <div class="request-status approved">{ "Approved" }</div>
                        </div>
                    </div>
                </div>
            }
        </section>
    }
}
