use gloo::{
    console::log,
    storage::{SessionStorage, Storage},
};
use wasm_bindgen_futures::spawn_local;
use yew::{
    function_component, html, use_context, use_state, Children, ContextProvider, Properties,
    UseStateHandle,
};

use super::access_token::new_access_token;
use crate::api::{models::Status, status as status_api, Client, ClientError};

#[derive(PartialEq, Clone)]
pub struct Context {
    pub access_token: UseStateHandle<Option<String>>,
    pub error: UseStateHandle<Option<ClientError>>,
}

impl Context {
    /// New api
    pub fn new() -> Self {
        let token: Option<String> = match SessionStorage::get("access_token") {
            Ok(t) => Some(t),
            Err(_) => None,
        };
        Self {
            access_token: use_state(move || token),
            error: use_state(move || None),
        }
    }

    /// Check if the token is valid and accept it
    pub fn authenticate(&mut self, token: &str) {
        log!(format!("auth token: {:?}", token));
        let token: String = token.trim().into();
        let client = Client::new(&token);
        {
            let access_token = self.access_token.clone();
            let error = self.error.clone();
            spawn_local(async move {
                match client.fetch::<Status>(status_api::read()).await {
                    Ok(s) => {
                        log!(format!("status: {:?}", s));
                        SessionStorage::set("access_token", token.clone())
                            .expect("session storage unavailable");
                        access_token.set(Some(token.clone()));
                        error.set(None);
                    }
                    Err(err) => {
                        log!(format!("Err: {:?}", err));
                        error.set(Some(err));
                    }
                }
            });
        }
    }

    /// Authentiate with jwt secret
    pub fn authenticate_secret(&mut self, secret: &str) {
        let token = new_access_token(secret);
        self.authenticate(&token)
    }

    /// Forget current session
    pub fn logout(&mut self) {
        SessionStorage::delete("access_token");
        self.access_token.set(None);
    }

    /// Helper to check if we are authenticated
    pub fn is_authenticated(&self) -> bool {
        match *self.access_token {
            Some(_) => true,
            None => false,
        }
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct AuthenticationContextProps {
    pub children: Children,
}

#[function_component(AuthenticationContext)]
pub fn authentication_context(props: &AuthenticationContextProps) -> Html {
    let AuthenticationContextProps { children } = props;
    let ctx = Context::new();
    html! {
        <ContextProvider<Context> context={ctx.clone()}>
            {for children.iter() }
        </ContextProvider<Context>>
    }
}

/// Use the authentication API
pub fn use_authentication() -> Context {
    let api = use_context::<Context>().expect("context missing");
    api
}

/// Retrieve the access token from the api state
pub fn use_access_token() -> Option<String> {
    let api = use_authentication();
    (*api.access_token).clone()
}
