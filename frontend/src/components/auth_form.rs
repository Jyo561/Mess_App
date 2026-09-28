use common::{LoginRequest, LoginResponse, SignupRequest};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::api;

#[derive(Properties, PartialEq)]
pub struct AuthFormProps {
    pub on_auth_success: Callback<LoginResponse>,
}

#[derive(Clone, Copy, PartialEq)]
enum AuthTab {
    SignIn,
    SignUp,
}

#[function_component(AuthForm)]
pub fn auth_form(props: &AuthFormProps) -> Html {
    let active_tab = use_state(|| AuthTab::SignIn);
    let error_msg = use_state(|| Option::<String>::None);
    let is_loading = use_state(|| false);

    // Common credentials
    let ic_no = use_state(String::new);
    let password = use_state(String::new);

    // Signup specific fields
    let name = use_state(String::new);
    let email = use_state(String::new);
    let designation = use_state(String::new);
    let ci = use_state(String::new);

    let switch_to_signin = {
        let active_tab = active_tab.clone();
        let error_msg = error_msg.clone();
        Callback::from(move |_| {
            active_tab.set(AuthTab::SignIn);
            error_msg.set(None);
        })
    };

    let switch_to_signup = {
        let active_tab = active_tab.clone();
        let error_msg = error_msg.clone();
        Callback::from(move |_| {
            active_tab.set(AuthTab::SignUp);
            error_msg.set(None);
        })
    };

    let make_input_handler = |state: UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                state.set(target.value());
            }
        })
    };

    let on_submit = {
        let active_tab = *active_tab;
        let ic_no = (*ic_no).clone();
        let password = (*password).clone();
        let name = (*name).clone();
        let email = (*email).clone();
        let designation = (*designation).clone();
        let ci = (*ci).clone();
        let is_loading = is_loading.clone();
        let error_msg = error_msg.clone();
        let on_success = props.on_auth_success.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            is_loading.set(true);
            error_msg.set(None);

            let ic_no = ic_no.clone();
            let password = password.clone();
            let name = name.clone();
            let email = email.clone();
            let designation = designation.clone();
            let ci = ci.clone();
            let is_loading = is_loading.clone();
            let error_msg = error_msg.clone();
            let on_success = on_success.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let result = match active_tab {
                    AuthTab::SignIn => {
                        api::login(LoginRequest {
                            ic_no: ic_no.trim().to_string(),
                            password,
                        })
                        .await
                    }
                    AuthTab::SignUp => {
                        match ic_no.trim().parse::<i32>() {
                            Ok(num_ic) => {
                                api::signup(SignupRequest {
                                    ic_no: num_ic,
                                    name,
                                    email,
                                    password,
                                    designation,
                                    ci,
                                })
                                .await
                            }
                            Err(_) => Err("IC No must be a numeric value (e.g. 1001)".to_string()),
                        }
                    }
                };

                is_loading.set(false);
                match result {
                    Ok(session) => on_success.emit(session),
                    Err(err) => error_msg.set(Some(err)),
                }
            });
        })
    };

    html! {
        <div class="auth-card">
            <div class="auth-tabs">
                <button
                    type="button"
                    class={if *active_tab == AuthTab::SignIn { "tab active" } else { "tab" }}
                    onclick={switch_to_signin}
                >
                    { "Sign In" }
                </button>
                <button
                    type="button"
                    class={if *active_tab == AuthTab::SignUp { "tab active" } else { "tab" }}
                    onclick={switch_to_signup}
                >
                    { "Register" }
                </button>
            </div>

            if let Some(ref err) = *error_msg {
                <div class="alert-banner">{ err }</div>
            }

            <form onsubmit={on_submit} class="auth-form">
                <div class="form-group">
                    <label>{ "IC Number" }</label>
                    <input
                        type="text"
                        placeholder="e.g. 1001"
                        value={(*ic_no).clone()}
                        oninput={make_input_handler(ic_no.clone())}
                        required=true
                    />
                </div>

                if *active_tab == AuthTab::SignUp {
                    <div class="form-group">
                        <label>{ "Full Name" }</label>
                        <input
                            type="text"
                            placeholder="Captain Vikram Batra"
                            value={(*name).clone()}
                            oninput={make_input_handler(name.clone())}
                            required=true
                        />
                    </div>

                    <div class="form-group">
                        <label>{ "Email Address" }</label>
                        <input
                            type="email"
                            placeholder="officer@enclave.mil"
                            value={(*email).clone()}
                            oninput={make_input_handler(email.clone())}
                            required=true
                        />
                    </div>

                    <div class="form-group">
                        <label>{ "Designation / Rank" }</label>
                        <input
                            type="text"
                            placeholder="e.g. Major / Adjutant"
                            value={(*designation).clone()}
                            oninput={make_input_handler(designation.clone())}
                            required=true
                        />
                    </div>

                    <div class="form-group">
                        <label>{ "CI / Unit" }</label>
                        <input
                            type="text"
                            placeholder="e.g. CI-Alpha"
                            value={(*ci).clone()}
                            oninput={make_input_handler(ci.clone())}
                            required=true
                        />
                    </div>
                }

                <div class="form-group">
                    <label>{ "Password" }</label>
                    <input
                        type="password"
                        placeholder="Enter password"
                        value={(*password).clone()}
                        oninput={make_input_handler(password.clone())}
                        required=true
                    />
                </div>

                <button type="submit" class="btn-primary" disabled={*is_loading}>
                    if *is_loading {
                        { "Processing..." }
                    } else if *active_tab == AuthTab::SignIn {
                        { "Sign In" }
                    } else {
                        { "Create Account" }
                    }
                </button>
            </form>
        </div>
    }
}
