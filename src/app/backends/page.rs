use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Properties};
use yew_router::{hooks::use_history, prelude::*};

use crate::{
    api::backends::{use_backend, use_backends},
    app::{backends::{form::Form, list::List}, Page, PageSelect, router::Route},
};

#[derive(Properties, Clone, PartialEq)]
pub struct BackendsPageProps {
    pub id: Option<String>,
}

#[function_component(BackendsPage)]
pub fn backends_page(props: &BackendsPageProps) -> Html {
    let BackendsPageProps { id } = props;
    let history = use_history().unwrap();
    let backends_ctx = use_backends();
    
    // Fetch specific backend if ID is provided
    let current_backend = use_state(|| None);
    
    // Always call use_backend but only use result when ID is provided
    // Use a dummy ID that won't cause API issues
    let dummy_id = "00000000-0000-0000-0000-000000000000".to_string();
    let backend_id = id.as_ref().unwrap_or(&dummy_id);
    let backend_state = use_backend(backend_id);
    
    
    {
        let current_backend = current_backend.clone();
        let backend_result = if id.is_some() {
            backend_state.result()
        } else {
            None
        };
        use_effect_with_deps(
            move |(id, backend_result)| {
                if let Some(backend) = backend_result {
                    current_backend.set(Some(backend.clone()));
                } else if id.is_some() {
                    // Keep loading state
                } else {
                    current_backend.set(None);
                }
                || ()
            },
            (id.clone(), backend_result),
        );
    }
    
    let on_save = {
        let history = history.clone();
        let backends_ctx = backends_ctx.clone();
        Callback::from(move |backend: b3scale_api::Backend| {
            // Refresh the list
            backends_ctx.fetch();
            // Navigate to the backend
            history.push(Route::Backends { id: backend.id.clone() });
        })
    };
    
    let on_delete = {
        let history = history.clone();
        let backends_ctx = backends_ctx.clone();
        Callback::from(move |_| {
            // Refresh the list
            backends_ctx.fetch();
            // Navigate to backends list
            history.push(Route::BackendsNew);
        })
    };
    
    let title = if let Some(backend) = &*current_backend {
        format!("{}: {}", backend.bbb.host, backend.id)
    } else if id.is_some() {
        "Loading...".to_string()
    } else {
        "Create New Backend".to_string()
    };
    
    
    html! {
        <Page>
            <aside>
                <PageSelect active="backends" />
                <List />
            </aside>
            <main class="container-flex">
                <div class="flex-row">
                    <h1>{title}</h1>
                    <div class="form-container backends col-md-8">
                        <Form 
                            backend={(*current_backend).clone()}
                            {on_save}
                            {on_delete}
                        />
                    </div>
                </div>
            </main>
        </Page>
    }
}
