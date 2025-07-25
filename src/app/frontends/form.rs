use gloo_console::log;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Properties, TargetCast};

use crate::components::CyberSlider;

use b3scale_api::{
    AttendeesLimitSettings, DefaultPresentationSettings, Frontend, FrontendConfig, 
    FrontendConfigPatch, FrontendPatch, FrontendRequest, FrontendSettings
};

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
    
    
    // Basic form state
    let key = use_state(|| frontend.as_ref().map(|f| f.bbb.key.clone()).unwrap_or_default());
    let secret = use_state(|| frontend.as_ref().map(|f| f.bbb.secret.clone()).unwrap_or_default());
    let active = use_state(|| frontend.as_ref().map(|f| f.active).unwrap_or(true));
    let account_ref = use_state(|| frontend.as_ref().and_then(|f| f.account_ref.clone()));
    
    // Frontend settings state
    let attendee_limit = use_state(|| {
        frontend.as_ref()
            .and_then(|f| f.settings.attendees_limit.as_ref())
            .map(|al| al.limit)
            .unwrap_or(100)
    });
    
    // Create default params (dynamic key-value pairs)
    let create_default_params = use_state(|| {
        frontend.as_ref()
            .map(|f| f.settings.create_default_params.clone())
            .unwrap_or_else(|| HashMap::new())
    });
    
    // Create override params (dynamic key-value pairs)  
    let create_override_params = use_state(|| {
        frontend.as_ref()
            .map(|f| f.settings.create_override_params.clone())
            .unwrap_or_else(|| HashMap::new())
    });
    
    // Default presentation settings
    let presentation_url = use_state(|| {
        frontend.as_ref()
            .and_then(|f| f.settings.default_presentation.as_ref())
            .map(|dp| dp.url.clone())
            .unwrap_or_default()
    });
    let presentation_force = use_state(|| {
        frontend.as_ref()
            .and_then(|f| f.settings.default_presentation.as_ref())
            .map(|dp| dp.force)
            .unwrap_or(false)
    });
    
    // Note: recordings settings were removed from the API
    
    // Required tags (dynamic list)
    let required_tags = use_state(|| {
        frontend.as_ref()
            .and_then(|f| f.settings.required_tags.as_ref())
            .cloned()
            .unwrap_or_else(|| Vec::new())
    });
    
    // Loading/error state
    let is_loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    
    // Update form fields when frontend prop changes
    {
        let frontend = frontend.clone();
        let key = key.clone();
        let secret = secret.clone();
        let active = active.clone();
        let account_ref = account_ref.clone();
        let attendee_limit = attendee_limit.clone();
        let create_default_params = create_default_params.clone();
        let create_override_params = create_override_params.clone();
        let presentation_url = presentation_url.clone();
        let presentation_force = presentation_force.clone();
        let required_tags = required_tags.clone();
        
        use_effect_with_deps(
            move |frontend_option| {
                if let Some(f) = frontend_option {
                    // Update basic fields
                    key.set(f.bbb.key.clone());
                    secret.set(f.bbb.secret.clone());
                    active.set(f.active);
                    account_ref.set(f.account_ref.clone());
                    
                    // Update settings fields
                    let limit = f.settings.attendees_limit.as_ref()
                        .map(|al| al.limit)
                        .unwrap_or(100);
                    attendee_limit.set(limit);
                    
                    create_default_params.set(f.settings.create_default_params.clone());
                    create_override_params.set(f.settings.create_override_params.clone());
                    
                    if let Some(dp) = &f.settings.default_presentation {
                        presentation_url.set(dp.url.clone());
                        presentation_force.set(dp.force);
                    } else {
                        presentation_url.set("".to_string());
                        presentation_force.set(false);
                    }
                    
                    if let Some(tags) = &f.settings.required_tags {
                        required_tags.set(tags.clone());
                    } else {
                        required_tags.set(Vec::new());
                    }
                } else {
                    // Reset to defaults for new frontend
                    key.set("".to_string());
                    secret.set("".to_string());
                    active.set(true);
                    account_ref.set(None);
                    attendee_limit.set(100);
                    create_default_params.set(HashMap::new());
                    create_override_params.set(HashMap::new());
                    presentation_url.set("".to_string());
                    presentation_force.set(false);
                    required_tags.set(Vec::new());
                }
                || ()
            },
            frontend.clone(),
        );
    }
    
    let is_edit_mode = frontend.is_some();
    
    // Input handlers for basic fields
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
    
    
    let on_account_ref_change = {
        let account_ref = account_ref.clone();
        Callback::from(move |e: yew::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let value = input.value();
                account_ref.set(if value.is_empty() { None } else { Some(value) });
            }
        })
    };
    
    // Attendee limit handler
    let on_attendee_limit_change = {
        let attendee_limit = attendee_limit.clone();
        Callback::from(move |e: yew::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<i32>() {
                    attendee_limit.set(value);
                }
            }
        })
    };
    
    // Presentation settings handlers
    let on_presentation_url_change = {
        let presentation_url = presentation_url.clone();
        Callback::from(move |e: yew::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                presentation_url.set(input.value());
            }
        })
    };
    
    
    // Note: recordings visibility handler removed - no longer in API
    
    // Helper to add new key-value pair to default params
    let add_default_param = {
        let create_default_params = create_default_params.clone();
        Callback::from(move |_| {
            let mut params = (*create_default_params).clone();
            params.insert("".to_string(), "".to_string());
            create_default_params.set(params);
        })
    };
    
    // Helper to add new key-value pair to override params
    let add_override_param = {
        let create_override_params = create_override_params.clone();
        Callback::from(move |_| {
            let mut params = (*create_override_params).clone();
            params.insert("".to_string(), "".to_string());
            create_override_params.set(params);
        })
    };
    
    // Helper to add new required tag
    let add_required_tag = {
        let required_tags = required_tags.clone();
        Callback::from(move |_| {
            let mut tags = (*required_tags).clone();
            tags.push("".to_string());
            required_tags.set(tags);
        })
    };
    
    // Submit handler
    let on_submit = {
        let client = client.clone();
        let key = key.clone();
        let secret = secret.clone();
        let active = active.clone();
        let account_ref = account_ref.clone();
        let attendee_limit = attendee_limit.clone();
        let create_default_params = create_default_params.clone();
        let create_override_params = create_override_params.clone();
        let presentation_url = presentation_url.clone();
        let presentation_force = presentation_force.clone();
        let required_tags = required_tags.clone();
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
            let attendee_limit = *attendee_limit;
            let create_default_params = (*create_default_params).clone();
            let create_override_params = (*create_override_params).clone();
            let presentation_url = (*presentation_url).clone();
            let presentation_force = *presentation_force;
            let required_tags = (*required_tags).clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            let frontend = frontend.clone();
            let on_save = on_save.clone();
            
            spawn_local(async move {
                is_loading.set(true);
                error.set(None);
                
                // Build the settings object (recordings field is ignored)
                let settings = FrontendSettings {
                    attendees_limit: Some(Box::new(AttendeesLimitSettings {
                        limit: attendee_limit,
                    })),
                    create_default_params,
                    create_override_params,
                    default_presentation: Some(Box::new(DefaultPresentationSettings {
                        url: presentation_url,
                        force: presentation_force,
                    })),
                    required_tags: Some(required_tags),
                    recordings: None, // Ignored field for backwards compatibility
                };
                
                let result = if let Some(existing) = &frontend {
                    // Update existing frontend
                    let patch = FrontendPatch {
                        active: Some(active),
                        account_ref: Some(account_ref),
                        bbb: Some(Box::new(FrontendConfigPatch {
                            key: Some(key),
                            secret: Some(secret),
                        })),
                        settings: Some(Box::new(settings)),
                    };
                    
                    client.fetch::<Frontend>(frontends::update(&existing.id, &patch)).await
                } else {
                    // Create new frontend
                    let request = FrontendRequest {
                        active: Some(active),
                        account_ref: Some(account_ref),
                        bbb: Box::new(FrontendConfig {
                            key,
                            secret,
                        }),
                        settings: Some(Box::new(settings)),
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
            
            // Basic frontend config
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
                    type="text"
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
                <CyberSlider 
                    checked={*active}
                    onchange={{
                        let active = active.clone();
                        Callback::from(move |checked| active.set(checked))
                    }}
                    disabled={Some(*is_loading)}
                    id={Some("activeSwitch".to_string())}
                    label={Some("Active".to_string())}
                />
            </div>
            
            // Attendee Limit Settings
            <section class="form-section">
                <h2>{"Attendee Limit"}</h2>
                <div class="form-group">
                    <label class="form-label">{"Maximum Attendees"}</label>
                    <input 
                        class="form-control" 
                        type="number"
                        value={attendee_limit.to_string()}
                        onchange={on_attendee_limit_change}
                        disabled={*is_loading}
                        min="1"
                    />
                </div>
            </section>
            
            // Create Default Parameters
            <section class="form-section">
                <h2>{"Create Default Parameters"}</h2>
                <p class="form-text">{"Key-value params used as defaults when creating meetings"}</p>
                
                {for create_default_params.iter().enumerate().map(|(i, (key_val, value_val))| {
                    let key_change = {
                        let create_default_params = create_default_params.clone();
                        let old_key = key_val.clone();
                        Callback::from(move |e: yew::Event| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                let mut params = (*create_default_params).clone();
                                if let Some(value) = params.remove(&old_key) {
                                    params.insert(input.value(), value);
                                }
                                create_default_params.set(params);
                            }
                        })
                    };
                    
                    let value_change = {
                        let create_default_params = create_default_params.clone();
                        let key_clone = key_val.clone();
                        Callback::from(move |e: yew::Event| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                let mut params = (*create_default_params).clone();
                                params.insert(key_clone.clone(), input.value());
                                create_default_params.set(params);
                            }
                        })
                    };
                    
                    let remove_param = {
                        let create_default_params = create_default_params.clone();
                        let key_clone = key_val.clone();
                        Callback::from(move |_| {
                            let mut params = (*create_default_params).clone();
                            params.remove(&key_clone);
                            create_default_params.set(params);
                        })
                    };
                    
                    html! {
                        <div class="row mb-2" key={i}>
                            <div class="form-group col-md-5">
                                <div class="input-group">
                                    <span class="input-group-text">{"Key"}</span>
                                    <input 
                                        class="form-control" 
                                        value={key_val.clone()}
                                        onchange={key_change}
                                        disabled={*is_loading}
                                    />
                                </div>
                            </div>
                            <div class="form-group col-md-5">
                                <div class="input-group">
                                    <span class="input-group-text">{"Value"}</span>
                                    <input 
                                        class="form-control" 
                                        value={value_val.clone()}
                                        onchange={value_change}
                                        disabled={*is_loading}
                                    />
                                </div>
                            </div>
                            <div class="col-md-2">
                                <button 
                                    type="button"
                                    class="btn btn-danger"
                                    onclick={remove_param}
                                    disabled={*is_loading}
                                >
                                    {"-"}
                                </button>
                            </div>
                        </div>
                    }
                })}
                
                <button 
                    type="button"
                    class="btn btn-secondary"
                    onclick={add_default_param}
                    disabled={*is_loading}
                >
                    {"+ Add Parameter"}
                </button>
            </section>
            
            // Create Override Parameters  
            <section class="form-section">
                <h2>{"Create Override Parameters"}</h2>
                <p class="form-text">{"Key-value params that override frontend params when creating meetings"}</p>
                
                {for create_override_params.iter().enumerate().map(|(i, (key_val, value_val))| {
                    let key_change = {
                        let create_override_params = create_override_params.clone();
                        let old_key = key_val.clone();
                        Callback::from(move |e: yew::Event| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                let mut params = (*create_override_params).clone();
                                if let Some(value) = params.remove(&old_key) {
                                    params.insert(input.value(), value);
                                }
                                create_override_params.set(params);
                            }
                        })
                    };
                    
                    let value_change = {
                        let create_override_params = create_override_params.clone();
                        let key_clone = key_val.clone();
                        Callback::from(move |e: yew::Event| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                let mut params = (*create_override_params).clone();
                                params.insert(key_clone.clone(), input.value());
                                create_override_params.set(params);
                            }
                        })
                    };
                    
                    let remove_param = {
                        let create_override_params = create_override_params.clone();
                        let key_clone = key_val.clone();
                        Callback::from(move |_| {
                            let mut params = (*create_override_params).clone();
                            params.remove(&key_clone);
                            create_override_params.set(params);
                        })
                    };
                    
                    html! {
                        <div class="row mb-2" key={i}>
                            <div class="form-group col-md-5">
                                <div class="input-group">
                                    <span class="input-group-text">{"Key"}</span>
                                    <input 
                                        class="form-control" 
                                        value={key_val.clone()}
                                        onchange={key_change}
                                        disabled={*is_loading}
                                    />
                                </div>
                            </div>
                            <div class="form-group col-md-5">
                                <div class="input-group">
                                    <span class="input-group-text">{"Value"}</span>
                                    <input 
                                        class="form-control" 
                                        value={value_val.clone()}
                                        onchange={value_change}
                                        disabled={*is_loading}
                                    />
                                </div>
                            </div>
                            <div class="col-md-2">
                                <button 
                                    type="button"
                                    class="btn btn-danger"
                                    onclick={remove_param}
                                    disabled={*is_loading}
                                >
                                    {"-"}
                                </button>
                            </div>
                        </div>
                    }
                })}
                
                <button 
                    type="button"
                    class="btn btn-secondary"
                    onclick={add_override_param}
                    disabled={*is_loading}
                >
                    {"+ Add Parameter"}
                </button>
            </section>
            
            // Required Tags
            <section class="form-section">
                <h2>{"Required Tags"}</h2>
                <p class="form-text">{"Only backends with all these tags will be considered for meetings"}</p>
                
                {for required_tags.iter().enumerate().map(|(i, tag)| {
                    let tag_change = {
                        let required_tags = required_tags.clone();
                        Callback::from(move |e: yew::Event| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                let mut tags = (*required_tags).clone();
                                tags[i] = input.value();
                                required_tags.set(tags);
                            }
                        })
                    };
                    
                    let remove_tag = {
                        let required_tags = required_tags.clone();
                        Callback::from(move |_| {
                            let mut tags = (*required_tags).clone();
                            tags.remove(i);
                            required_tags.set(tags);
                        })
                    };
                    
                    html! {
                        <div class="row mb-2" key={i}>
                            <div class="form-group col-md-10">
                                <div class="input-group">
                                    <span class="input-group-text">{"Tag"}</span>
                                    <input 
                                        class="form-control" 
                                        value={tag.clone()}
                                        onchange={tag_change}
                                        disabled={*is_loading}
                                    />
                                </div>
                            </div>
                            <div class="col-md-2">
                                <button 
                                    type="button"
                                    class="btn btn-danger"
                                    onclick={remove_tag}
                                    disabled={*is_loading}
                                >
                                    {"-"}
                                </button>
                            </div>
                        </div>
                    }
                })}
                
                <button 
                    type="button"
                    class="btn btn-secondary"
                    onclick={add_required_tag}
                    disabled={*is_loading}
                >
                    {"+ Add Tag"}
                </button>
            </section>
            
            // Default Presentation
            <section class="form-section">
                <h2>{"Default Presentation"}</h2>
                <div class="form-group">
                    <label class="form-label">{"Presentation URL"}</label>
                    <input 
                        class="form-control" 
                        type="url"
                        value={(*presentation_url).clone()}
                        onchange={on_presentation_url_change}
                        disabled={*is_loading}
                        placeholder="https://example.com/presentation.pdf"
                    />
                </div>
                
                <CyberSlider 
                    checked={*presentation_force}
                    onchange={{
                        let presentation_force = presentation_force.clone();
                        Callback::from(move |checked| presentation_force.set(checked))
                    }}
                    disabled={Some(*is_loading)}
                    id={Some("presentationForceSwitch".to_string())}
                    label={Some("Force Override (override any presentation from frontend)".to_string())}
                />
            </section>
            
            
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
        </form>
    }
}