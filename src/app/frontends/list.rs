use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, TargetCast, Html};
use yew_router::{hooks::use_navigator, prelude::*};

use crate::{
    api::frontends::use_frontends,
    app::router::Route,
};

#[function_component(List)]
pub fn list() -> Html {
    let history = use_navigator().unwrap();
    let frontends_ctx = use_frontends();
    let search_term = use_state(|| String::new());
    
    let on_search_input = {
        let search_term = search_term.clone();
        Callback::from(move |e: yew::InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                search_term.set(input.value());
            }
        })
    };
    
    if frontends_ctx.is_loading() {
        return html! {
            <nav class="nav-list">
                <div class="nav-content">
                    <div class="loading">{"Loading..."}</div>
                </div>
            </nav>
        };
    }
    
    if let Some(error) = frontends_ctx.error() {
        return html! {
            <nav class="nav-list">
                <div class="nav-content">
                    <div class="alert alert-danger">
                        {format!("Error loading frontends: {}", error)}
                    </div>
                </div>
            </nav>
        };
    }
    
    let all_frontends = frontends_ctx.result().unwrap_or_default();
    
    // Filter frontends based on search term
    let filtered_frontends: Vec<_> = if search_term.is_empty() {
        all_frontends.clone()
    } else {
        let search_lower = search_term.to_lowercase();
        all_frontends
            .iter()
            .filter(|frontend| {
                frontend.bbb.key.to_lowercase().contains(&search_lower) ||
                frontend.id.to_lowercase().contains(&search_lower) ||
                frontend.account_ref.as_ref().unwrap_or(&String::new()).to_lowercase().contains(&search_lower)
            })
            .cloned()
            .collect()
    };
    
    html! {
        <nav class="nav-list">
            <div class="nav-content">
                // Search field
                <div class="mb-3">
                    <input 
                        type="text"
                        class="form-control form-control-sm"
                        placeholder="Search frontends..."
                        value={(*search_term).clone()}
                        oninput={on_search_input}
                    />
                </div>
                
                <ul class="nav nav-pills nav-fill flex-column">
                    {for filtered_frontends.iter().map(|frontend| {
                        let frontend_id = frontend.id.clone();
                        let on_select = {
                            let history = history.clone();
                            let frontend_id = frontend_id.clone();
                            Callback::from(move |_| {
                                history.push(&Route::Frontends { id: frontend_id.clone() });
                            })
                        };
                        
                        html! {
                            <li class="nav-item" key={frontend.id.clone()}>
                                <a 
                                    class="nav-link" 
                                    href="#"
                                    onclick={on_select}
                                >
                                    <div class="d-flex justify-content-between align-items-start">
                                        <div>
                                            <div class="fw-bold">{&frontend.bbb.key}</div>
                                            <div class="text-muted small">
                                                {format!("ID: {}...", &frontend.id[..8])}
                                            </div>
                                            {if let Some(ref account_ref) = frontend.account_ref {
                                                html! {
                                                    <div class="text-muted small">
                                                        {"Account: "}{account_ref}
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }}
                                        </div>
                                        <span class={if frontend.active { "badge bg-success" } else { "badge bg-secondary" }}>
                                            {if frontend.active { "Active" } else { "Inactive" }}
                                        </span>
                                    </div>
                                </a>
                            </li>
                        }
                    })}
                    
                    if filtered_frontends.is_empty() {
                        <li class="nav-item">
                            <div class="nav-link text-muted">
                                {if search_term.is_empty() {
                                    "No frontends found"
                                } else {
                                    "No frontends match your search"
                                }}
                            </div>
                        </li>
                    }
                </ul>
            </div>
        </nav>
    }
}
