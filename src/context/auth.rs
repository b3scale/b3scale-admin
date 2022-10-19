use anyhow::Result;
use gloo::storage::{SessionStorage, Storage};

use yew::{
    function_component, html, use_context, use_state, Children, ContextProvider, Properties,
    UseStateHandle,
};

#[derive(PartialEq, Clone)]
pub struct Context {
    pub access_token: UseStateHandle<Option<String>>,
}

impl Context {
    /// New api
    pub fn new() -> Self {
        let token = match SessionStorage::get::<String>("access_token") {
            Ok(t) => Some(t),
            Err(_) => None,
        };
        Self {
            access_token: use_state(move || token),
        }
    }

    /// Check if the token is valid and accept it
    pub fn authenticate(&mut self, token: &str) -> Result<()> {
        let token: String = token.into();
        SessionStorage::set("access_token", token.clone())?;
        self.access_token.set(Some(token.to_owned()));
        Ok(())
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
