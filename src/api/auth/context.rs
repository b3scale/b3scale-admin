use gloo::{
    console::log,
    storage::{SessionStorage, Storage},
};
use wasm_bindgen_futures::spawn_local;
use yew::{
    function_component, html, use_context, use_state, hook, Children, ContextProvider, Html, Properties,
    UseStateHandle,
};

use super::access_token::new_access_token;
use crate::api::{status as status_api, Client, ClientError};
use b3scale_api::Status;

#[derive(PartialEq, Clone)]
pub struct Context {
    pub access_token: UseStateHandle<Option<String>>,
    pub api_url: UseStateHandle<Option<String>>,
    pub error: UseStateHandle<Option<ClientError>>,
}

impl Context {
    /// New api
    pub fn new(
        access_token: UseStateHandle<Option<String>>,
        api_url: UseStateHandle<Option<String>>,
        error: UseStateHandle<Option<ClientError>>,
    ) -> Self {
        Self {
            access_token,
            api_url,
            error,
        }
    }

    /// Check if the token is valid and accept it
    pub fn authenticate(&mut self, token: &str, api_url: &str) {
        log!(format!("auth token: {:?}", token));
        log!(format!("api url: {:?}", api_url));
        let token: String = token.trim().into();
        let api_url_str = if api_url.is_empty() { 
            None 
        } else { 
            Some(api_url.trim().to_string())
        };
        let client = Client::new(&token);
        {
            let access_token = self.access_token.clone();
            let api_url_state = self.api_url.clone();
            let error = self.error.clone();
            let api_url_clone = api_url_str.clone();
            spawn_local(async move {
                match client.fetch::<Status>(status_api::read_with_base_url(api_url_clone.as_deref())).await {
                    Ok(s) => {
                        log!(format!("status: {:?}", s));
                        SessionStorage::set("access_token", token.clone())
                            .expect("session storage unavailable");
                        if let Some(url) = &api_url_clone {
                            SessionStorage::set("api_url", url)
                                .expect("session storage unavailable");
                        } else {
                            SessionStorage::delete("api_url");
                        }
                        access_token.set(Some(token.clone()));
                        api_url_state.set(api_url_clone);
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
    pub fn authenticate_secret(&mut self, secret: &str, api_url: &str) {
        let token = new_access_token(secret);
        self.authenticate(&token, api_url)
    }

    /// Forget current session
    pub fn logout(&mut self) {
        SessionStorage::delete("access_token");
        SessionStorage::delete("api_url");
        self.access_token.set(None);
        self.api_url.set(None);
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
    
    let token: Option<String> = match SessionStorage::get("access_token") {
        Ok(t) => Some(t),
        Err(_) => None,
    };
    
    let api_url: Option<String> = match SessionStorage::get("api_url") {
        Ok(url) => Some(url),
        Err(_) => None,
    };
    
    let access_token = use_state(move || token);
    let api_url_state = use_state(move || api_url);
    let error = use_state(|| None);
    let ctx = Context::new(access_token, api_url_state, error);
    
    html! {
        <ContextProvider<Context> context={ctx.clone()}>
            {for children.iter() }
        </ContextProvider<Context>>
    }
}

/// Use the authentication API
#[hook]
pub fn use_authentication() -> Context {
    let api = use_context::<Context>().expect("context missing");
    api
}

/// Retrieve the access token from the api state
#[hook]
pub fn use_access_token() -> Option<String> {
    let api = use_authentication();
    (*api.access_token).clone()
}

/// Retrieve the API URL from the api state
#[hook]
pub fn use_api_url() -> Option<String> {
    let api = use_authentication();
    (*api.api_url).clone()
}
