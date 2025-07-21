use gloo_console;
use yew::{function_component, html, Callback};
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
    
    // Debug logging
    gloo_console::log!("Backends loading:", backends_ctx.is_loading());
    if let Some(error) = backends_ctx.error() {
        gloo_console::log!("Backends error:", format!("{:?}", error));
    }
    if let Some(result) = backends_ctx.result() {
        gloo_console::log!("Backends result length:", result.len());
    }
    
    let on_create = {
        let history = history.clone();
        Callback::from(move |_| {
            history.push(Route::BackendsNew);
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
    
    let backends = backends_ctx.result().unwrap_or_default();
    
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
                <ul class="nav nav-pills nav-fill flex-column">
                    {for backends.iter().map(|backend| {
                        let backend_id = backend.id.clone();
                        let on_select = {
                            let history = history.clone();
                            let backend_id = backend_id.clone();
                            Callback::from(move |_| {
                                gloo_console::log!("Selecting backend:", &backend_id);
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
                    
                    if backends.is_empty() {
                        <li class="nav-item">
                            <div class="nav-link text-muted">
                                {"No backends found"}
                            </div>
                        </li>
                    }
                </ul>
            </div>
        </nav>
    }
}