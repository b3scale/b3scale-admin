use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, TargetCast};
use yew_router::{hooks::use_history, prelude::*};

use b3scale_api::backend::{AdminState, NodeState};
use crate::{
    api::backends::use_backends,
    app::router::Route,
};

#[function_component(List)]
pub fn list() -> Html {
    let history = use_history().unwrap();
    let backends_ctx = use_backends();
    let search_term = use_state(|| String::new());
    
    let on_create = {
        let history = history.clone();
        Callback::from(move |_| {
            history.push(Route::BackendsNew);
        })
    };
    
    let on_search_input = {
        let search_term = search_term.clone();
        Callback::from(move |e: yew::InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                search_term.set(input.value());
            }
        })
    };
    
    if backends_ctx.is_loading() {
        return html! {
            <nav class="nav-list">
                <div class="nav-header">
                    <h2>{"Backends"}</h2>
                </div>
                <div class="nav-content">
                    <div class="loading">{"Loading..."}</div>
                </div>
            </nav>
        };
    }
    
    if let Some(error) = backends_ctx.error() {
        return html! {
            <nav class="nav-list">
                <div class="nav-header">
                    <h2>{"Backends"}</h2>
                </div>
                <div class="nav-content">
                    <div class="alert alert-danger">
                        {format!("Error loading backends: {}", error)}
                    </div>
                </div>
            </nav>
        };
    }
    
    let all_backends = backends_ctx.result().unwrap_or_default();
    
    // Filter backends based on search term
    let filtered_backends: Vec<_> = if search_term.is_empty() {
        all_backends.clone()
    } else {
        let search_lower = search_term.to_lowercase();
        all_backends
            .iter()
            .filter(|backend| {
                backend.bbb.host.to_lowercase().contains(&search_lower) ||
                backend.id.to_lowercase().contains(&search_lower) ||
                backend.agent_ref.as_ref().unwrap_or(&String::new()).to_lowercase().contains(&search_lower) ||
                backend.settings.tags.iter().any(|tag| tag.to_lowercase().contains(&search_lower))
            })
            .cloned()
            .collect()
    };
    
    html! {
        <nav class="nav-list">
            <div class="nav-header">
                <h2>{"Backends"}</h2>
                <button 
                    type="button" 
                    class="btn btn-primary btn-sm"
                    onclick={on_create}
                >
                    {"+ Add Backend"}
                </button>
            </div>
            <div class="nav-content">
                // Search field
                <div class="mb-3">
                    <input 
                        type="text"
                        class="form-control form-control-sm"
                        placeholder="Search backends..."
                        value={(*search_term).clone()}
                        oninput={on_search_input}
                    />
                </div>
                
                <ul class="nav nav-pills nav-fill flex-column">
                    {for filtered_backends.iter().map(|backend| {
                        let backend_id = backend.id.clone();
                        let on_select = {
                            let history = history.clone();
                            let backend_id = backend_id.clone();
                            Callback::from(move |_| {
                                history.push(Route::Backends { id: backend_id.clone() });
                            })
                        };
                        
                        // Determine the status badge based on admin_state and node_state
                        let (status_class, status_text) = match (&backend.admin_state, &backend.node_state) {
                            (_, NodeState::Ready) => ("badge bg-success", "Ready"),
                            (_, NodeState::Error) => ("badge bg-danger", "Error"),
                            (_, NodeState::Stopped) => ("badge bg-secondary", "Stopped"),
                            (AdminState::Decommissioned, _) => ("badge bg-warning", "Decommissioned"),
                            (_, NodeState::Init) => ("badge bg-info", "Initializing"),
                            _ => ("badge bg-secondary", "Unknown"),
                        };
                        
                        html! {
                            <li class="nav-item" key={backend.id.clone()}>
                                <a 
                                    class="nav-link" 
                                    href="#"
                                    onclick={on_select}
                                >
                                    <div class="d-flex justify-content-between align-items-start">
                                        <div>
                                            <div class="fw-bold">{&backend.bbb.host}</div>
                                            <div class="text-muted small">
                                                {format!("Load: {:.1} | Meetings: {} | Attendees: {}", 
                                                    backend.load_factor, 
                                                    backend.meetings_count, 
                                                    backend.attendees_count
                                                )}
                                            </div>
                                            {if !backend.settings.tags.is_empty() {
                                                html! {
                                                    <div class="text-muted small">
                                                        {"Tags: "}{backend.settings.tags.join(", ")}
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }}
                                        </div>
                                        <span class={status_class}>{status_text}</span>
                                    </div>
                                </a>
                            </li>
                        }
                    })}
                    
                    if filtered_backends.is_empty() {
                        <li class="nav-item">
                            <div class="nav-link text-muted">
                                {if search_term.is_empty() {
                                    "No backends found"
                                } else {
                                    "No backends match your search"
                                }}
                            </div>
                        </li>
                    }
                </ul>
            </div>
        </nav>
    }
}