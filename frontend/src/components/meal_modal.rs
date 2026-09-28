use common::MealType;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MealModalProps {
    pub meal: Option<MealType>,
    pub on_confirm: Callback<MealType>,
    pub on_cancel: Callback<()>,
}

#[function_component(MealModal)]
pub fn meal_modal(props: &MealModalProps) -> Html {
    let Some(meal) = props.meal else {
        return html! {};
    };

    let icon = match meal {
        MealType::Breakfast => "🍳",
        MealType::Lunch => "🍛",
        MealType::Dinner => "🍽",
    };

    let on_overlay_click = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };

    let on_card_click = Callback::from(|e: MouseEvent| e.stop_propagation());

    let on_confirm_click = {
        let on_confirm = props.on_confirm.clone();
        Callback::from(move |_| on_confirm.emit(meal))
    };

    let on_cancel_click = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };

    html! {
        <div class="modal-overlay show" onclick={on_overlay_click}>
            <div class="modal neu" onclick={on_card_click}>
                <div class="modal-icon neu-small">{ icon }</div>
                <h2>{ format!("Mark {:?}", meal) }</h2>
                <p>{ format!("Are you having {:?} today?", meal) }</p>
                <div class="modal-actions">
                    <button class="primary-btn" onclick={on_confirm_click}>
                        { "Yes, I'm in" }
                    </button>
                    <button class="secondary-btn" onclick={on_cancel_click}>
                        { "Cancel" }
                    </button>
                </div>
            </div>
        </div>
    }
}
