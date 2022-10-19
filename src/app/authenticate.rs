use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::{
    events::{FocusEvent, KeyboardEvent},
    function_component, html, use_node_ref, Callback, NodeRef,
};

fn use_input_ref(node: &NodeRef) -> String {
    let node = node.clone();
    if let Some(input) = node.cast::<HtmlInputElement>() {
        input.value()
    } else {
        "".into()
    }
}

#[function_component(Form)]
fn form() -> Html {
    let token_ref = use_node_ref();
    let secret_ref = use_node_ref();

    let on_submit = {
        let token_ref = token_ref.clone();
        let secret_ref = secret_ref.clone();
        Callback::from(move |ev: FocusEvent| {
            ev.prevent_default();
            let token = use_input_ref(&token_ref);
            let secret = use_input_ref(&secret_ref);
            log!(format!("on submit: {:?} - {:?}", token, secret));
        })
    };

    let on_secret_changed = {
        let secret_ref = secret_ref.clone();
        Callback::from(move |_: KeyboardEvent| {
            let token = use_input_ref(&secret_ref);
            log!(format!("secret changed: {:?}", token));
        })
    };

    let on_token_changed = {
        let token_ref = token_ref.clone();
        Callback::from(move |_: KeyboardEvent| {
            let token = use_input_ref(&token_ref);
            log!(format!("token changed: {:?}", token));
        })
    };

    html! {
      <form onsubmit={on_submit}>
        <div class="form-group">
        <label for="token">{"Paste your access token:"}
        </label>
        <textarea ref={token_ref} onkeyup={on_token_changed} name="token" class="form-control"></textarea>
        </div>
      <hr />
        <div class="form-group">
        <label for="secret">{"Enter the API JWT secret:"}</label>
        <input ref={secret_ref} onkeyup={on_secret_changed} type="text" class="form-control" name="secret" />
        </div>
      </form>
    }
}

#[function_component(Authenticate)]
pub fn authenticate() -> Html {
    html! {
        <div class="
            container-page
            d-flex
            justify-content-center
            align-items-center">

        <div class="card box box-authenticate">
            <div class="card-body">
              <Form />
            </div>
        </div>

        </div>
    }
}
