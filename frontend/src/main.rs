use yew::prelude::*;
use gloo_net::http::Request;
use common::{MealType,AttendanceEntry};
// use crate::event::Event;
use chrono::{Datelike, Local};
use wasm_bindgen_futures::spawn_local; // Import the async executor
use gloo_dialogs::alert; // Import the alert box

#[function_component(App)]
fn app() -> Html {
    let meals = use_state(|| vec![]);
    
    // Fetch meals from your Poem backend
    {
        let meals = meals.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched: Vec<MealType> = Request::get("http://localhost:3000/api/meals")
                    .send().await.unwrap().json().await.unwrap();
                meals.set(fetched);
            });
        });
    }

    let on_submit = |meal: MealType| {
        Callback::from(move |_| {
            // In a real app, use_state to track input values per meal
            let entry = AttendanceEntry {
                id: None,
                user_id: "user123".to_string(), // Hardcoded for today
                meal,
                date: chrono::Local::now().date_naive(),
                response: true, 
                remarks: "Test".to_string(),
            };

            wasm_bindgen_futures::spawn_local(async move {
                let _ = Request::post("http://localhost:3000/api/entry")
                    .json(&entry).unwrap()
                    .send().await;
                alert("Submitted!");
            });
        })
    };

    html! {
        <div style="width: 100%; max-width: 400px; padding: 20px;">
            <h1 style="text-align: center;">{"Enclave Mess"}</h1>
            { for meals.iter().map(|meal| {
                let meal_clone = meal.clone();
                html! {
                    <div class="neumorphic" style="margin-bottom: 20px;">
                        <h3>{ format!("{:?}", meal_clone) }</h3>
                        <button onclick={on_submit(meal_clone)} 
                                style="width: 100%; padding: 10px; background: #e0e0e0;">
                            {"Submit Yes"}
                        </button>
                    </div>
                }
            })}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
