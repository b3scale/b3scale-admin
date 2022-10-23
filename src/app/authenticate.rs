use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::{
    events::{FocusEvent, KeyboardEvent},
    function_component, html, use_effect, use_node_ref, use_state, Callback, NodeRef, Properties,
};
use yew_router::{history::History, hooks::use_history};

use crate::{api::auth::use_authentication, app::router::Route};

fn use_input_ref(node: &NodeRef) -> Option<String> {
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
    let show_submit = use_state(|| false);

    let auth_error = (*auth.error).clone();

    let on_submit = {
        let on_submit_cb = on_submit_cb.clone();
        let token_ref = token_ref.clone();
        let secret_ref = secret_ref.clone();
        Callback::from(move |ev: FocusEvent| {
            ev.prevent_default();
            let token = use_input_ref(&token_ref);
            let secret = use_input_ref(&secret_ref);
            on_submit_cb.emit(FormData {
                token: token.clone(),
                secret: secret.clone(),
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
        let on_submit_cb = on_submit_cb.clone();
        Callback::from(move |_: KeyboardEvent| {
            let token = use_input_ref(&token_ref);
            if let Some(token) = token {
                on_submit_cb.emit(FormData {
                    secret: None,
                    token: Some(token.clone()),
                })
            }
        })
    };

    html! {
      <form onsubmit={on_submit}>
        if let Some(err) = auth_error {
            <div class="alert alert-danger error auth-error">
                {err.message()}
            </div>
        }
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
                auth.authenticate(&token);
            } else if let Some(secret) = f.secret {
                auth.authenticate_secret(&secret);
            }
        })
    };

    {
        let history = use_history().unwrap();
        let auth = auth.clone();
        use_effect(move || {
            // Navigate to start if authenticated
            if auth.is_authenticated() {
                history.replace(Route::Start);
            }
            || ()
        });
    }

    html! {
        <div class="
            container-page
            d-flex
            justify-content-center
            align-items-center">

        <div class="card box box-authenticate">
            <div class="card-body">
              <Form {on_submit} />
            </div>
        </div>

        </div>
    }
}
