use yew::{function_component, html, Properties};

use crate::app::{frontends::list::List, Link, Page, PageSelect, Route};

#[function_component(Form)]
pub fn form() -> Html {
    html! {
        <form class="form">
            <div class="form-group">
               <label class="form-label">{"Key"}</label>
               <input class="form-control" />
            </div>
            <div class="form-group">
               <label class="form-label">{"Secret"}</label>
               <input class="form-control" />
            </div>
            <div class="form-group form-section">
                <div class="form-check form-switch">
                      <label class="form-check-label" for="disabledSwitchCheckChecked">
                      <input class="form-check-input" type="checkbox" id="disabledSwitchCheckChecked" checked={false} />
                        {"Active"}
                      </label>
                </div>
            </div>

            <section class="form-section">
              <h2>{"Create Default Parameters"}</h2>

               <div class="row">
                <div class="form-group col-md-5">
                  <div class="input-group">
                    <span class="input-group-text">{"Key"}</span>
                    <input class="form-control" />
                  </div>
                </div>
                <div class="form-group col-md-5">
                  <div class="input-group">
                    <span class="input-group-text">{"Value"}</span>
                    <input class="form-control" />
                  </div>
                </div>

                <div class="col-md-2 align-right">
                  <div class="btn-group">
                   <button class="btn btn-dark form-control">{"+"}</button>
                   <button disabled={true} class="btn btn-dark form-control">{"-"}</button>
                  </div>
                </div>
               </div>
               <div class="row">
                <div class="form-group col-md-5">
                  <div class="input-group">
                    <span class="input-group-text">{"Key"}</span>
                    <input class="form-control" />
                  </div>
                </div>
                <div class="form-group col-md-5">
                  <div class="input-group">
                    <span class="input-group-text">{"Value"}</span>
                    <input class="form-control" />
                  </div>
                </div>

                <div class="col-md-2 align-right">
                  <div class="btn-group">
                   <button class="btn btn-dark form-control">{"+"}</button>
                   <button disabled={false} class="btn btn-dark form-control">{"-"}</button>
                  </div>
                </div>
               </div>
            </section>

            <section class="form-section">
              <h2>{"Create Override Parameters"}</h2>

               <div class="row">
                <div class="form-group col-md-5">
                  <div class="input-group">
                    <span class="input-group-text">{"Key"}</span>
                    <input class="form-control" />
                  </div>
                </div>
                <div class="form-group col-md-5">
                  <div class="input-group">
                    <span class="input-group-text">{"Value"}</span>
                    <input class="form-control" />
                  </div>
                </div>

                <div class="col-md-2 align-right">
                  <div class="btn-group">
                   <button class="btn btn-dark form-control">{"+"}</button>
                   <button disabled={true} class="btn btn-dark form-control">{"-"}</button>
                  </div>
                </div>
               </div>
            </section>

            <section class="form-section">

            </section>

            <section class="form-section">
              <h2>{"Required Tags"}</h2>

                <div class="row">

                    <div class="form-group col-md-10">
                      <div class="input-group">
                        <span class="input-group-text">{"Tag"}</span>
                        <input class="form-control" />
                      </div>
                    </div>

                    <div class="col-md-2 align-right">
                      <div class="btn-group">
                       <button class="btn btn-dark form-control">{"+"}</button>
                       <button disabled={true} class="btn btn-dark form-control">{"-"}</button>
                      </div>
                    </div>

                </div>

            </section>

            <section class="form-section">
                <h2>{"Default Presentation"}</h2>
                <div class="form-group">
                  <div class="input-group">
                    <span class="input-group-text">{"URL"}</span>
                    <input class="form-control" />
                  </div>
                </div>

                <div class="form-check form-switch">
                      <label class="form-check-label" for="disabledSwitchCheckChecked">
                      <input class="form-check-input" type="checkbox" id="disabledSwitchCheckChecked" checked={false} />
                        {"Force Override"}
                      </label>
                </div>

            </section>

        </form>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct FrontendsPageProps {
    pub id: Option<String>,
}

#[function_component(FrontendsPage)]
pub fn frontends_page(props: &FrontendsPageProps) -> Html {
    html! {
        <Page>
          <aside>
            <PageSelect active="frontends" />
            <List />

          </aside>
          <main class="container-flex">
            <div class="flex-row">
                <h1>{"green42: 70c53ba9-f9d2-4720-a06e-abfcc61cae9a"}</h1>
                <div class="form-container frontends col-md-6">

                  <Form />

                </div>
            </div>
          </main>
        </Page>
    }
}
