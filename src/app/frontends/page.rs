use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Properties};
use yew_router::{hooks::use_history, prelude::*};

use crate::{
    api::frontends::{use_frontend, use_frontends},
    app::{frontends::{form::Form, list::List}, Page, PageSelect, router::Route},
};

#[derive(Properties, Clone, PartialEq)]
pub struct FrontendsPageProps {
    pub id: Option<String>,
}

#[function_component(FrontendsPage)]
pub fn frontends_page(props: &FrontendsPageProps) -> Html {
    let FrontendsPageProps { id } = props;
    let history = use_history().unwrap();
    let frontends_ctx = use_frontends();
    
    // Fetch specific frontend if ID is provided
    let current_frontend = use_state(|| None);
    
    // Always call use_frontend but only use result when ID is provided
    // Use a dummy ID that won't cause API issues
    let dummy_id = "00000000-0000-0000-0000-000000000000".to_string();
    let frontend_id = id.as_ref().unwrap_or(&dummy_id);
    let frontend_state = use_frontend(frontend_id);
    
    
    {
        let current_frontend = current_frontend.clone();
        let frontend_result = if id.is_some() {
            frontend_state.result()
        } else {
            None
        };
        use_effect_with_deps(
            move |(id, frontend_result)| {
                if let Some(frontend) = frontend_result {
                    current_frontend.set(Some(frontend.clone()));
                } else if id.is_some() {
                    // Keep loading state
                } else {
                    current_frontend.set(None);
                }
                || ()
            },
            (id.clone(), frontend_result),
        );
    }
    
    let on_save = {
        let history = history.clone();
        let frontends_ctx = frontends_ctx.clone();
        Callback::from(move |frontend: b3scale_api::Frontend| {
            // Refresh the list
            frontends_ctx.fetch();
            // Navigate to the frontend
            history.push(Route::Frontends { id: frontend.id.clone() });
        })
    };
    
    let on_delete = {
        let history = history.clone();
        let frontends_ctx = frontends_ctx.clone();
        Callback::from(move |_| {
            // Refresh the list
            frontends_ctx.fetch();
            // Navigate to frontends list
            history.push(Route::FrontendsNew);
        })
    };
    
    let title = if let Some(frontend) = &*current_frontend {
        format!("{}: {}", frontend.bbb.key, frontend.id)
    } else if id.is_some() {
        "Loading...".to_string()
    } else {
        "Create New Frontend".to_string()
    };
    
    
    html! {
        <Page>
            <aside>
                <PageSelect active="frontends" />
                <List />
            </aside>
            <main class="container-flex">
                <div class="flex-row">
                    <h1>{title}</h1>
                    <div class="form-container frontends col-md-8">
                        <Form 
                            frontend={(*current_frontend).clone()}
                            {on_save}
                            {on_delete}
                        />
                    </div>
                </div>
            </main>
        </Page>
    }
}
