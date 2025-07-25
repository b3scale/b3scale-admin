use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::{
    events::{KeyboardEvent, SubmitEvent},
    function_component, html, use_effect, use_node_ref, use_state, Callback, NodeRef, Properties,
    Html,
};
use yew_router::hooks::use_navigator;

use crate::{api::auth::use_authentication, app::router::Route};

fn get_input_value(node: &NodeRef) -> Option<String> {
    let node = node.clone();
    match node.cast::<HtmlInputElement>() {
        Some(input) => {
            let value = input.value();
            if value == "" {
                None
            } else {
                Some(value)
            }
        }
        None => None,
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FormData {
    pub token: Option<String>,
    pub secret: Option<String>,
    pub api_url: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct FormProps {
    pub on_submit: Callback<FormData>,
}

#[function_component(Form)]
fn form(props: &FormProps) -> Html {
    let auth = use_authentication();

    let on_submit_cb = props.on_submit.clone();
    let token_ref = use_node_ref();
    let secret_ref = use_node_ref();
    let api_url_ref = use_node_ref();
    let show_submit = use_state(|| false);

    let auth_error = (*auth.error).clone();

    let on_submit = {
        let on_submit_cb = on_submit_cb.clone();
        let token_ref = token_ref.clone();
        let secret_ref = secret_ref.clone();
        let api_url_ref = api_url_ref.clone();
        Callback::from(move |ev: SubmitEvent| {
            ev.prevent_default();
            let token = get_input_value(&token_ref);
            let secret = get_input_value(&secret_ref);
            let api_url = get_input_value(&api_url_ref).unwrap_or_else(|| "".to_string());
            on_submit_cb.emit(FormData {
                token: token.clone(),
                secret: secret.clone(),
                api_url,
            });
        })
    };

    let on_secret_changed = {
        let show_submit = show_submit.clone();
        Callback::from(move |_: KeyboardEvent| {
            show_submit.set(true);
        })
    };

    let on_token_changed = {
        let token_ref = token_ref.clone();
        let api_url_ref = api_url_ref.clone();
        let on_submit_cb = on_submit_cb.clone();
        Callback::from(move |_: KeyboardEvent| {
            let token = get_input_value(&token_ref);
            let api_url = get_input_value(&api_url_ref).unwrap_or_else(|| "".to_string());
            if let Some(token) = token {
                on_submit_cb.emit(FormData {
                    secret: None,
                    token: Some(token.clone()),
                    api_url,
                })
            }
        })
    };

    html! {
      <form onsubmit={on_submit}>
        if let Some(err) = auth_error {
            <div class="alert alert-danger error auth-error">
                <strong>{"Authentication failed:"}</strong>
                <br />
                {err.message()}
                {
                    // Add helpful hints based on error type
                    if err.message().contains("CORS") {
                        html! {
                            <div class="mt-2">
                                <small class="text-muted">
                                    {"💡 "}
                                    <strong>{"Fix: "}</strong>
                                    {"Add this domain to the CORS allowed origins in your b3scale API configuration."}
                                </small>
                            </div>
                        }
                    } else if err.message().contains("Cannot connect") || err.message().contains("NetworkError") || err.message().contains("fetch") {
                        html! {
                            <div class="mt-2">
                                <small class="text-muted">
                                    {"💡 "}
                                    <strong>{"Troubleshooting: "}</strong>
                                    {"1) Verify the API URL is correct, 2) Check if the server is running, 3) Try accessing the API URL directly in your browser."}
                                </small>
                            </div>
                        }
                    } else if err.message().contains("401") || err.message().contains("Unauthorized") {
                        html! {
                            <div class="mt-2">
                                <small class="text-muted">
                                    {"💡 "}
                                    <strong>{"Fix: "}</strong>
                                    {"Double-check your access token or generate a new one from your b3scale API server."}
                                </small>
                            </div>
                        }
                    } else if err.message().contains("404") {
                        html! {
                            <div class="mt-2">
                                <small class="text-muted">
                                    {"💡 "}
                                    <strong>{"Fix: "}</strong>
                                    {"The API endpoint was not found. Check if you're using the correct API URL and version."}
                                </small>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
        <div class="form-group">
        <label for="api_url">{"API URL:"}</label>
        <input ref={api_url_ref} type="text" class="form-control" name="api_url" placeholder="https://api.example.com or leave empty for same origin" />
        <small class="form-text text-muted">
            <strong>{"💡 Tip:"}</strong>
            {" Leave empty to use same origin, or specify full URL (e.g. https://api.yourdomain.com) to connect to external API server."}
        </small>
        </div>
        <br />
        <div class="form-group">
        <label for="token">{"Paste your access token:"}
        </label>
        <textarea ref={token_ref} onkeyup={on_token_changed}  name="token" class="form-control"></textarea>
        </div>
      <hr />
        <div class="form-group">
        <label for="secret">{"Enter the API JWT secret:"}</label>
        <input ref={secret_ref} onkeyup={on_secret_changed} type="password" class="form-control" name="secret" />
        </div>
        if *show_submit {
            <br />
            <div class="d-flex flex-column align-items-end">
              <button class="btn btn-success" type="submit">{"Login"}</button>
            </div>
        }
      </form>
    }
}

#[function_component(AuthenticatePage)]
pub fn authenticate_page() -> Html {
    let auth = use_authentication();
    let on_submit = {
        let auth = auth.clone();
        Callback::from(move |f: FormData| {
            let mut auth = auth.clone();
            log!(format!("form: {:?}", f));
            if let Some(token) = f.token {
                auth.authenticate(&token, &f.api_url);
            } else if let Some(secret) = f.secret {
                auth.authenticate_secret(&secret, &f.api_url);
            }
        })
    };

    {
        let navigator = use_navigator().unwrap();
        let auth = auth.clone();
        use_effect(move || {
            // Navigate to start if authenticated
            if auth.is_authenticated() {
                navigator.replace(&Route::Start);
            }
            || ()
        });
    }

    html! {
        <div class="vh-100 d-flex">
        <div class="
            vh-100
            container-page
            d-flex
            justify-content-center
            align-items-center">

        <div class="card box box-authenticate cyber-matrix matrix-scanlines matrix-text-rain">
            <div class="card-body">
              <Form {on_submit} />
            </div>
        </div>
        </div>
        </div>
    }
}
