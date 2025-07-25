use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_effect_with, use_state, Callback, Properties, TargetCast, Html};

use b3scale_api::{
    backend::{AdminState, NodeState},
    backend_patch::AdminState as PatchAdminState,
    backend_request::AdminState as RequestAdminState,
    Backend, BackendConfig, BackendPatch, BackendRequest, BackendSettings,
};

use crate::api::{
    backends,
    client::use_client,
};

#[derive(Properties, Clone, PartialEq)]
pub struct FormProps {
    pub backend: Option<Backend>,
    pub on_save: Option<Callback<Backend>>,
    pub on_delete: Option<Callback<()>>,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let FormProps { backend, on_save, on_delete } = props;
    let client = use_client();
    
    
    // Basic form state
    let host = use_state(|| backend.as_ref().map(|b| b.bbb.host.clone()).unwrap_or_default());
    let secret = use_state(|| backend.as_ref().map(|b| b.bbb.secret.clone()).unwrap_or_default());
    let admin_state = use_state(|| backend.as_ref().map(|b| b.admin_state).unwrap_or(AdminState::Ready));
    let load_factor = use_state(|| backend.as_ref().map(|b| b.load_factor).unwrap_or(1.0));
    
    // Backend settings state - tags list
    let tags = use_state(|| {
        backend.as_ref()
            .map(|b| b.settings.tags.clone())
            .unwrap_or_else(|| Vec::new())
    });
    
    // Loading/error state
    let is_loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    
    // Update form fields when backend prop changes
    {
        let backend = backend.clone();
        let host = host.clone();
        let secret = secret.clone();
        let admin_state = admin_state.clone();
        let load_factor = load_factor.clone();
        let tags = tags.clone();
        
        use_effect_with(
            backend.clone(),
            move |backend_option| {
                if let Some(b) = backend_option {
                    // Update basic fields
                    host.set(b.bbb.host.clone());
                    secret.set(b.bbb.secret.clone());
                    admin_state.set(b.admin_state);
                    load_factor.set(b.load_factor);
                    
                    // Update settings fields
                    tags.set(b.settings.tags.clone());
                } else {
                    // Reset to defaults for new backend
                    host.set("".to_string());
                    secret.set("".to_string());
                    admin_state.set(AdminState::Ready);
                    load_factor.set(1.0);
                    tags.set(Vec::new());
                }
                || ()
            },
        );
    }
    
    let is_edit_mode = backend.is_some();
    
    // Input handlers for basic fields
    let on_host_change = {
        let host = host.clone();
        Callback::from(move |e: yew::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                host.set(input.value());
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
    
    let on_admin_state_change = {
        let admin_state = admin_state.clone();
        Callback::from(move |e: yew::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let state = match input.value().as_str() {
                    "init" => AdminState::Init,
                    "ready" => AdminState::Ready,
                    "stopped" => AdminState::Stopped,
                    "decommissioned" => AdminState::Decommissioned,
                    _ => AdminState::Ready,
                };
                admin_state.set(state);
            }
        })
    };
    
    let on_load_factor_change = {
        let load_factor = load_factor.clone();
        Callback::from(move |e: yew::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    load_factor.set(value);
                }
            }
        })
    };
    
    // Helper to add new tag
    let add_tag = {
        let tags = tags.clone();
        Callback::from(move |_| {
            let mut tag_list = (*tags).clone();
            tag_list.push("".to_string());
            tags.set(tag_list);
        })
    };
    
    // Submit handler
    let on_submit = {
        let client = client.clone();
        let host = host.clone();
        let secret = secret.clone();
        let admin_state = admin_state.clone();
        let load_factor = load_factor.clone();
        let tags = tags.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let backend = backend.clone();
        let on_save = on_save.clone();
        
        Callback::from(move |e: web_sys::SubmitEvent| {
            e.prevent_default();
            
            let client = client.clone();
            let host = (*host).clone();
            let secret = (*secret).clone();
            let admin_state = *admin_state;
            let load_factor = *load_factor;
            let tags = (*tags).clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            let backend = backend.clone();
            let on_save = on_save.clone();
            
            spawn_local(async move {
                is_loading.set(true);
                error.set(None);
                
                let settings = BackendSettings {
                    tags,
                };
                
                let result = if let Some(existing) = &backend {
                    // Update existing backend
                    let patch_admin_state = match admin_state {
                        AdminState::Init => PatchAdminState::Init,
                        AdminState::Ready => PatchAdminState::Ready,
                        AdminState::Stopped => PatchAdminState::Stopped,
                        AdminState::Decommissioned => PatchAdminState::Decommissioned,
                    };
                    
                    let patch = BackendPatch {
                        admin_state: Some(patch_admin_state),
                        bbb: Some(Box::new(BackendConfig {
                            host,
                            secret,
                        })),
                        load_factor: Some(load_factor),
                        settings: Some(Box::new(settings)),
                        ..Default::default()
                    };
                    
                    client.fetch::<Backend>(backends::update(&existing.id, &patch)).await
                } else {
                    // Create new backend
                    let request_admin_state = match admin_state {
                        AdminState::Init => RequestAdminState::Init,
                        AdminState::Ready => RequestAdminState::Ready,
                        AdminState::Stopped => RequestAdminState::Stopped,
                        AdminState::Decommissioned => RequestAdminState::Decommissioned,
                    };
                    
                    let request = BackendRequest {
                        admin_state: Some(request_admin_state),
                        bbb: Box::new(BackendConfig {
                            host,
                            secret,
                        }),
                        load_factor: Some(load_factor),
                        settings: Some(Box::new(settings)),
                    };
                    
                    client.fetch::<Backend>(backends::create(&request)).await
                };
                
                match result {
                    Ok(backend) => {
                        log!("Backend saved successfully");
                        if let Some(on_save) = on_save {
                            on_save.emit(backend);
                        }
                    }
                    Err(err) => {
                        log!(format!("Error saving backend: {:?}", err));
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
        let backend = backend.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let on_delete = on_delete.clone();
        
        Callback::from(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            
            if let Some(backend) = &backend {
                if !web_sys::window()
                    .unwrap()
                    .confirm_with_message(&format!("Delete backend '{}'?", backend.bbb.host))
                    .unwrap_or(false)
                {
                    return;
                }
                
                let client = client.clone();
                let id = backend.id.clone();
                let is_loading = is_loading.clone();
                let error = error.clone();
                let on_delete = on_delete.clone();
                
                spawn_local(async move {
                    is_loading.set(true);
                    error.set(None);
                    
                    match client.fetch::<()>(backends::delete(&id)).await {
                        Ok(_) => {
                            log!("Backend deleted successfully");
                            if let Some(on_delete) = on_delete {
                                on_delete.emit(());
                            }
                        }
                        Err(err) => {
                            log!(format!("Error deleting backend: {:?}", err));
                            error.set(Some(format!("Failed to delete: {}", err)));
                        }
                    }
                    
                    is_loading.set(false);
                });
            }
        })
    };
    
    html! {
        <>
            // Backend Stats & Info Block (only shown in edit mode)
            if let Some(backend_data) = backend {
                <div class="card mb-4">
                    <div class="card-body">
                        <div class="row">
                            <div class="col-md-6">
                                <div class="mb-3">
                                    <strong>{"Backend ID:"}</strong>
                                    <div class="text-muted font-monospace small">{&backend_data.id}</div>
                                </div>
                                
                                <div class="mb-3">
                                    <strong>{"Node State:"}</strong>
                                    <div class="d-flex align-items-center">
                                        {match backend_data.node_state {
                                            NodeState::Ready => html! {
                                                <span class="badge bg-success ms-2">{"Ready"}</span>
                                            },
                                            NodeState::Error => html! {
                                                <span class="badge bg-danger ms-2">{"Error"}</span>
                                            },
                                            NodeState::Stopped => html! {
                                                <span class="badge bg-secondary ms-2">{"Stopped"}</span>
                                            },
                                            NodeState::Init => html! {
                                                <span class="badge bg-info ms-2">{"Initializing"}</span>
                                            },
                                            NodeState::Decommissioned => html! {
                                                <span class="badge bg-warning ms-2">{"Decommissioned"}</span>
                                            },
                                        }}
                                    </div>
                                </div>
                                
                                <div class="mb-3">
                                    <strong>{"Latency:"}</strong>
                                    <div class="text-muted">
                                        {format!("{} ms", backend_data.latency)}
                                    </div>
                                </div>
                                
                                <div class="mb-3">
                                    <strong>{"Agent Reference:"}</strong>
                                    <div class="text-muted">
                                        {backend_data.agent_ref.as_ref().unwrap_or(&"None".to_string())}
                                    </div>
                                </div>
                            </div>
                            
                            <div class="col-md-6">
                                <div class="mb-3">
                                    <strong>{"Current Load:"}</strong>
                                    <div class="text-muted">
                                        {format!("Meetings: {} | Attendees: {}", backend_data.meetings_count, backend_data.attendees_count)}
                                    </div>
                                </div>
                                
                                <div class="mb-3">
                                    <strong>{"Last Heartbeat:"}</strong>
                                    <div class="text-muted small">
                                        {&backend_data.agent_heartbeat}
                                    </div>
                                </div>
                                
                                <div class="mb-3">
                                    <strong>{"Last Sync:"}</strong>
                                    <div class="text-muted small">
                                        {&backend_data.synced_at}
                                    </div>
                                </div>
                                
                                <div class="mb-3">
                                    <strong>{"Created:"}</strong>
                                    <div class="text-muted small">
                                        {&backend_data.created_at}
                                    </div>
                                </div>
                                
                                <div class="mb-0">
                                    <strong>{"Updated:"}</strong>
                                    <div class="text-muted small">
                                        {&backend_data.updated_at}
                                    </div>
                                </div>
                            </div>
                        </div>
                        
                        // Last Error (prominent display if present)
                        if let Some(ref last_error) = backend_data.last_error {
                            <div class="alert alert-warning mt-3" role="alert">
                                <strong>{"Last Error:"}</strong>
                                <div class="mt-1">{last_error}</div>
                            </div>
                        }
                    </div>
                </div>
            }
        
            <form class="form" onsubmit={on_submit}>
                if let Some(err) = &*error {
                    <div class="alert alert-danger" role="alert">
                        {err}
                    </div>
                }
            
            // Basic backend config
            <div class="form-group">
                <label class="form-label">{"Host URL"}</label>
                <input 
                    class="form-control" 
                    type="url"
                    value={(*host).clone()}
                    onchange={on_host_change}
                    required={true}
                    disabled={*is_loading}
                    placeholder="https://bbb.example.com/bigbluebutton/api/"
                />
                <div class="form-text">{"Full qualified address including API endpoint"}</div>
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
                <div class="form-text">{"API secret for the BBB host"}</div>
            </div>
            
            <div class="form-group">
                <label class="form-label">{"Admin State"}</label>
                <select 
                    class="form-select" 
                    onchange={on_admin_state_change}
                    disabled={*is_loading}
                >
                    <option value="init" selected={matches!(*admin_state, AdminState::Init)}>
                        {"Init"}
                    </option>
                    <option value="ready" selected={matches!(*admin_state, AdminState::Ready)}>
                        {"Ready"}
                    </option>
                    <option value="stopped" selected={matches!(*admin_state, AdminState::Stopped)}>
                        {"Stopped"}
                    </option>
                    <option value="decommissioned" selected={matches!(*admin_state, AdminState::Decommissioned)}>
                        {"Decommissioned"}
                    </option>
                </select>
                <div class="form-text">{"Desired state of the backend node"}</div>
            </div>
            
            <div class="form-group">
                <label class="form-label">{"Load Factor"}</label>
                <input 
                    class="form-control" 
                    type="number"
                    step="0.1"
                    min="0.1"
                    max="10.0"
                    value={load_factor.to_string()}
                    onchange={on_load_factor_change}
                    disabled={*is_loading}
                />
                <div class="form-text">{"Influences selection probability (multiplier for meetings/attendees)"}</div>
            </div>
            
            // Tags
            <section class="form-section">
                <h2>{"Tags"}</h2>
                <p class="form-text">{"Backend provides these tags - frontends can require specific tags"}</p>
                
                {for tags.iter().enumerate().map(|(i, tag)| {
                    let tag_change = {
                        let tags = tags.clone();
                        Callback::from(move |e: yew::Event| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                let mut tag_list = (*tags).clone();
                                tag_list[i] = input.value();
                                tags.set(tag_list);
                            }
                        })
                    };
                    
                    let remove_tag = {
                        let tags = tags.clone();
                        Callback::from(move |_| {
                            let mut tag_list = (*tags).clone();
                            tag_list.remove(i);
                            tags.set(tag_list);
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
                    onclick={add_tag}
                    disabled={*is_loading}
                >
                    {"+ Add Tag"}
                </button>
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
                        {"Update Backend"}
                    } else {
                        {"Create Backend"}
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
        </>
    }
}
