use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Properties, TargetCast};

use b3scale_api::{Frontend, FrontendPatch, FrontendRequest};

use crate::api::{
    client::use_client,
    frontends,
};

#[derive(Properties, Clone, PartialEq)]
pub struct FormProps {
    pub frontend: Option<Frontend>,
    pub on_save: Option<Callback<Frontend>>,
    pub on_delete: Option<Callback<()>>,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let FormProps { frontend, on_save, on_delete } = props;
    let client = use_client();
    
    // Form state
    let key = use_state(|| frontend.as_ref().map(|f| f.bbb.key.clone()).unwrap_or_default());
    let secret = use_state(|| frontend.as_ref().map(|f| f.bbb.secret.clone()).unwrap_or_default());
    let active = use_state(|| frontend.as_ref().map(|f| f.active).unwrap_or(true));
    let account_ref = use_state(|| frontend.as_ref().and_then(|f| f.account_ref.clone()));
    
    // Loading/error state
    let is_loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    
    let is_edit_mode = frontend.is_some();
    
    // Input handlers
    let on_key_change = {
        let key = key.clone();
        Callback::from(move |e: yew::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                key.set(input.value());
            }
        })
    };
    
    let on_secret_change = {
        let secret = secret.clone();
        Callback::from(move |e: yew::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                secret.set(input.value());
            }
        })
    };
    
    let on_active_change = {
        let active = active.clone();
        Callback::from(move |e: yew::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                active.set(input.checked());
            }
        })
    };
    
    let on_account_ref_change = {
        let account_ref = account_ref.clone();
        Callback::from(move |e: yew::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let value = input.value();
                account_ref.set(if value.is_empty() { None } else { Some(value) });
            }
        })
    };
    
    // Submit handler
    let on_submit = {
        let client = client.clone();
        let key = key.clone();
        let secret = secret.clone();
        let active = active.clone();
        let account_ref = account_ref.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let frontend = frontend.clone();
        let on_save = on_save.clone();
        
        Callback::from(move |e: web_sys::FocusEvent| {
            e.prevent_default();
            
            let client = client.clone();
            let key = (*key).clone();
            let secret = (*secret).clone();
            let active = *active;
            let account_ref = (*account_ref).clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            let frontend = frontend.clone();
            let on_save = on_save.clone();
            
            spawn_local(async move {
                is_loading.set(true);
                error.set(None);
                
                let result = if let Some(existing) = &frontend {
                    // Update existing frontend
                    let patch = FrontendPatch {
                        active: Some(active),
                        account_ref: Some(account_ref),
                        bbb: Some(Box::new(b3scale_api::FrontendConfigPatch {
                            key: Some(key),
                            secret: Some(secret),
                        })),
                        settings: None,
                    };
                    
                    client.fetch::<Frontend>(frontends::update(&existing.id, &patch)).await
                } else {
                    // Create new frontend
                    let request = FrontendRequest {
                        active: Some(active),
                        account_ref: Some(account_ref),
                        bbb: Box::new(b3scale_api::FrontendConfig {
                            key,
                            secret,
                        }),
                        settings: None,
                    };
                    
                    client.fetch::<Frontend>(frontends::create(&request)).await
                };
                
                match result {
                    Ok(frontend) => {
                        log!("Frontend saved successfully");
                        if let Some(on_save) = on_save {
                            on_save.emit(frontend);
                        }
                    }
                    Err(err) => {
                        log!(format!("Error saving frontend: {:?}", err));
                        error.set(Some(format!("Failed to save: {}", err)));
                    }
                }
                
                is_loading.set(false);
            });
        })
    };
    
    // Delete handler
    let on_delete_click = {
        let client = client.clone();
        let frontend = frontend.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let on_delete = on_delete.clone();
        
        Callback::from(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            
            if let Some(frontend) = &frontend {
                if !web_sys::window()
                    .unwrap()
                    .confirm_with_message(&format!("Delete frontend '{}'?", frontend.bbb.key))
                    .unwrap_or(false)
                {
                    return;
                }
                
                let client = client.clone();
                let id = frontend.id.clone();
                let is_loading = is_loading.clone();
                let error = error.clone();
                let on_delete = on_delete.clone();
                
                spawn_local(async move {
                    is_loading.set(true);
                    error.set(None);
                    
                    match client.fetch::<()>(frontends::delete(&id)).await {
                        Ok(_) => {
                            log!("Frontend deleted successfully");
                            if let Some(on_delete) = on_delete {
                                on_delete.emit(());
                            }
                        }
                        Err(err) => {
                            log!(format!("Error deleting frontend: {:?}", err));
                            error.set(Some(format!("Failed to delete: {}", err)));
                        }
                    }
                    
                    is_loading.set(false);
                });
            }
        })
    };
    
    html! {
        <form class="form" onsubmit={on_submit}>
            if let Some(err) = &*error {
                <div class="alert alert-danger" role="alert">
                    {err}
                </div>
            }
            
            <div class="form-group">
                <label class="form-label">{"Key"}</label>
                <input 
                    class="form-control" 
                    value={(*key).clone()}
                    onchange={on_key_change}
                    required={true}
                    disabled={*is_loading}
                />
            </div>
            
            <div class="form-group">
                <label class="form-label">{"Secret"}</label>
                <input 
                    class="form-control" 
                    type="password"
                    value={(*secret).clone()}
                    onchange={on_secret_change}
                    required={true}
                    disabled={*is_loading}
                />
            </div>
            
            <div class="form-group">
                <label class="form-label">{"Account Reference (optional)"}</label>
                <input 
                    class="form-control" 
                    value={(*account_ref).clone().unwrap_or_default()}
                    onchange={on_account_ref_change}
                    disabled={*is_loading}
                    placeholder="Optional account reference"
                />
            </div>
            
            <div class="form-group form-section">
                <div class="form-check form-switch">
                    <input 
                        class="form-check-input" 
                        type="checkbox" 
                        id="activeSwitch"
                        checked={*active}
                        onchange={on_active_change}
                        disabled={*is_loading}
                    />
                    <label class="form-check-label" for="activeSwitch">
                        {"Active"}
                    </label>
                </div>
            </div>
            
            <div class="form-group mt-4">
                <button 
                    type="submit" 
                    class="btn btn-primary me-2"
                    disabled={*is_loading}
                >
                    if *is_loading {
                        {"Saving..."}
                    } else if is_edit_mode {
                        {"Update Frontend"}
                    } else {
                        {"Create Frontend"}
                    }
                </button>
                
                if is_edit_mode {
                    <button 
                        type="button"
                        class="btn btn-danger"
                        onclick={on_delete_click}
                        disabled={*is_loading}
                    >
                        {"Delete"}
                    </button>
                }
            </div>
            
            // TODO: Add sections for:
            // - Create Default Parameters
            // - Create Override Parameters  
            // - Required Tags
            // - Default Presentation
            // These require more complex state management with dynamic lists
        </form>
    }
}