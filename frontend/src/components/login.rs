use common::{LoginRequest, LoginResponse, SignupRequest};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::api;

#[derive(Clone, Copy, PartialEq)]
enum AuthMode {
    SignIn,
    SignUp,
}

#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub on_login_success: Callback<LoginResponse>,
    pub show_toast: Callback<String>,
}

#[function_component(LoginView)]
pub fn login_view(props: &LoginProps) -> Html {
    let mode = use_state(|| AuthMode::SignIn);
    let is_loading = use_state(|| false);

    // Shared credentials
    let ic_no = use_state(String::new);
    let password = use_state(String::new);

    // Sign Up specific fields
    let name = use_state(String::new);
    let email = use_state(String::new);
    let designation = use_state(String::new);
    let ci = use_state(String::new);

    // Generic input handler generator
    let make_input_handler = |handle: UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                handle.set(target.value());
            }
        })
    };

    let on_submit = {
        let mode = *mode;
        let ic_no = ic_no.clone();
        let password = password.clone();
        let name = name.clone();
        let email = email.clone();
        let designation = designation.clone();
        let ci = ci.clone();
        let is_loading = is_loading.clone();
        let show_toast = props.show_toast.clone();
        let on_success = props.on_login_success.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if ic_no.trim().is_empty() || password.trim().is_empty() {
                show_toast.emit("Please enter your IC number and password".into());
                return;
            }

            is_loading.set(true);

            let ic_str = (*ic_no).clone();
            let pwd = (*password).clone();
            let name_str = (*name).clone();
            let email_str = (*email).clone();
            let desig_str = (*designation).clone();
            let ci_str = (*ci).clone();

            let is_loading = is_loading.clone();
            let show_toast = show_toast.clone();
            let on_success = on_success.clone();

            spawn_local(async move {
                match mode {
                    AuthMode::SignIn => {
                        match api::login(LoginRequest { ic_no: ic_str, password: pwd }).await {
                            Ok(res) => {
                                show_toast.emit("Signed in successfully".into());
                                on_success.emit(res);
                            }
                            Err(err) => show_toast.emit(format!("Sign in failed: {err}")),
                        }
                    }
                    AuthMode::SignUp => {
                        // Numeric IC validation
                        match ic_str.trim().parse::<i32>() {
                            Ok(numeric_ic) => {
                                let req = SignupRequest {
                                    ic_no: numeric_ic,
                                    name: name_str,
                                    email: email_str,
                                    password: pwd,
                                    designation: desig_str,
                                    ci: ci_str,
                                };

                                match api::signup(req).await {
                                    Ok(res) => {
                                        show_toast.emit("Account created & signed in!".into());
                                        on_success.emit(res);
                                    }
                                    Err(err) => show_toast.emit(format!("Registration failed: {err}")),
                                }
                            }
                            Err(_) => {
                                show_toast.emit("IC Number must be numeric (e.g. 1001)".into());
                            }
                        }
                    }
                }
                is_loading.set(false);
            });
        })
    };

    html! {
        <div class="login-screen">
            <div class={classes!("login-card", "neu", if *mode == AuthMode::SignUp { "signup-card-expand" } else { "" })}>
                <div class="logo neu-small">{ "🏠" }</div>
                <h1>{ "Enclave" }</h1>
                <p>{ "Mess Management System" }</p>

                // Neumorphic Segmented Tab Switcher
                <div class="tabs neu-inset" style="margin-bottom: 24px;">
                    <button
                        type="button"
                        class={if *mode == AuthMode::SignIn { "tab active" } else { "tab" }}
                        onclick={{
                            let mode = mode.clone();
                            Callback::from(move |_| mode.set(AuthMode::SignIn))
                        }}
                    >
                        { "Sign In" }
                    </button>
                    <button
                        type="button"
                        class={if *mode == AuthMode::SignUp { "tab active" } else { "tab" }}
                        onclick={{
                            let mode = mode.clone();
                            Callback::from(move |_| mode.set(AuthMode::SignUp))
                        }}
                    >
                        { "Register" }
                    </button>
                </div>

                <form onsubmit={on_submit}>
                    if *mode == AuthMode::SignIn {
                        // ---------------------------------------------
                        // Sign In Fields
                        // ---------------------------------------------
                        <div class="input-group">
                            <label>{ "IC Number / Service ID" }</label>
                            <input
                                class="input neu-inset"
                                type="text"
                                placeholder="e.g. 1001"
                                value={(*ic_no).clone()}
                                oninput={make_input_handler(ic_no.clone())}
                                required=true
                            />
                        </div>

                        <div class="input-group">
                            <label>{ "Password" }</label>
                            <input
                                class="input neu-inset"
                                type="password"
                                placeholder="Enter your password"
                                value={(*password).clone()}
                                oninput={make_input_handler(password.clone())}
                                required=true
                            />
                        </div>
                    } else {
                        // ---------------------------------------------
                        // Sign Up (Register) Fields
                        // ---------------------------------------------
                        <div class="input-grid-2">
                            <div class="input-group">
                                <label>{ "IC Number" }</label>
                                <input
                                    class="input neu-inset"
                                    type="number"
                                    placeholder="e.g. 1002"
                                    value={(*ic_no).clone()}
                                    oninput={make_input_handler(ic_no.clone())}
                                    required=true
                                />
                            </div>

                            <div class="input-group">
                                <label>{ "Full Name" }</label>
                                <input
                                    class="input neu-inset"
                                    type="text"
                                    placeholder="Capt Vikram Batra"
                                    value={(*name).clone()}
                                    oninput={make_input_handler(name.clone())}
                                    required=true
                                />
                            </div>
                        </div>

                        <div class="input-group">
                            <label>{ "Service Email" }</label>
                            <input
                                class="input neu-inset"
                                type="email"
                                placeholder="officer@enclave.mil"
                                value={(*email).clone()}
                                oninput={make_input_handler(email.clone())}
                                required=true
                            />
                        </div>

                        <div class="input-grid-2">
                            <div class="input-group">
                                <label>{ "Rank / Designation" }</label>
                                <input
                                    class="input neu-inset"
                                    type="text"
                                    placeholder="Captain / Adjutant"
                                    value={(*designation).clone()}
                                    oninput={make_input_handler(designation.clone())}
                                    required=true
                                />
                            </div>

                            <div class="input-group">
                                <label>{ "CI / Unit" }</label>
                                <input
                                    class="input neu-inset"
                                    type="text"
                                    placeholder="CI-Alpha / 13 JAK RIF"
                                    value={(*ci).clone()}
                                    oninput={make_input_handler(ci.clone())}
                                    required=true
                                />
                            </div>
                        </div>

                        <div class="input-group">
                            <label>{ "Create Password" }</label>
                            <input
                                class="input neu-inset"
                                type="password"
                                placeholder="At least 6 characters"
                                value={(*password).clone()}
                                oninput={make_input_handler(password.clone())}
                                required=true
                            />
                        </div>
                    }

                    <button class="primary-btn" type="submit" disabled={*is_loading} style="margin-top: 10px;">
                        if *is_loading {
                            { "Processing..." }
                        } else if *mode == AuthMode::SignIn {
                            { "Sign In" }
                        } else {
                            { "Create Service Account" }
                        }
                    </button>
                </form>

                // Secondary toggle link
                <div style="margin-top: 20px;">
                    if *mode == AuthMode::SignIn {
                        <button
                            type="button"
                            class="auth-link"
                            onclick={{
                                let mode = mode.clone();
                                Callback::from(move |_| mode.set(AuthMode::SignUp))
                            }}
                        >
                            { "New to the Mess? Register an account →" }
                        </button>
                    } else {
                        <button
                            type="button"
                            class="auth-link"
                            onclick={{
                                let mode = mode.clone();
                                Callback::from(move |_| mode.set(AuthMode::SignIn))
                            }}
                        >
                            { "Already registered? Sign In →" }
                        </button>
                    }
                </div>

                <div class="login-footer">
                    { "Secure JWT authentication · PWA" }
                </div>
            </div>
        </div>
    }
}
