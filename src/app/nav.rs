use yew::{
    classes, events::MouseEvent, function_component, html, Callback, Children, Classes, Properties,
};
use yew_router::{
    history::History,
    hooks::{use_history, use_route},
    Routable,
};

use std::iter::zip;

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps<T: Routable> {
    pub children: Children,
    pub to: T,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(99)]
    pub depth: usize,
}

/// Check if the prefixes match
fn match_prefix<T: Routable>(a: T, b: T, depth: usize) -> bool {
    // Compare
    let match_depth = a
        .to_path()
        .split('/')
        .zip(b.to_path().split('/'))
        .map(|(a, b)| if a == b { 1 } else { 0 })
        .fold(0, |acc, v| acc + v);
    depth <= match_depth
}

#[function_component(Link)]
pub fn link<T: Routable + 'static>(props: &LinkProps<T>) -> Html {
    let LinkProps {
        children,
        to,
        class,
        depth,
    } = props.clone();
    let history = use_history().unwrap();
    let route: T = use_route().unwrap();
    let active = match_prefix(route.clone(), to.clone(), depth);
    let href = to.to_owned().to_path();

    let on_navigate = {
        let history = history.clone();
        let target = to.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history.push(target.clone());
        })
    };

    html! {
      <a onclick={on_navigate}
        class={classes!(
         "nav-link",
         active.then(|| Some("active")),
         class.clone(),
       )} href={href}>
        { for children.iter() }
      </a>
    }
}

#[function_component(Button)]
pub fn button<T: Routable + 'static>(props: &LinkProps<T>) -> Html {
    let LinkProps {
        children,
        to,
        class,
        ..
    } = props;
    let history = use_history().unwrap();
    let route: T = use_route().unwrap();
    let active = &route == to;
    let href = to.to_owned().to_path();

    let on_navigate = {
        let history = history.clone();
        let target = to.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history.push(target.clone());
        })
    };

    html! {
      <button onclick={on_navigate}
        class={classes!(
         "list-group-item",
         "list-group-item-action",
         active.then(|| Some("active")),
         class.clone(),
       )} href={href}>
        { for children.iter() }
      </button>
    }
}
