use yew::{
    classes, events::MouseEvent, function_component, html, Callback, Children, Classes, Properties,
    Html,
};
use yew_router::{
    history::History,
    hooks::{use_navigator, use_route},
    Routable,
};

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps<T: Routable> {
    pub children: Children,
    pub to: T,
    #[prop_or_default]
    pub class: Classes,
    pub active: Option<bool>,
}

fn match_prefix<T: Routable>(a: &T, b: &T) -> bool {
    let a: Vec<String> = a.to_path().split("/").map(|p| p.to_owned()).collect();
    let b: Vec<String> = b.to_path().split("/").map(|p| p.to_owned()).collect();
    let mut b = b.iter();
    for a_ in a {
        if let Some(b_) = b.next() {
            if &a_ == b_ {
                continue;
            }
        } else {
            return false;
        }
    }
    true
}

#[function_component(Link)]
pub fn link<T: Routable + 'static>(props: &LinkProps<T>) -> Html {
    let LinkProps {
        children,
        to,
        class,
        active,
    } = props.clone();
    let history = use_navigator().unwrap();
    let route: T = use_route().unwrap();
    let active = if let Some(a) = active {
        a
    } else {
        match_prefix(&route, &to)
    };
    let href = to.to_owned().to_path();

    let on_navigate = {
        let history = history.clone();
        let target = to.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history.push(&target);
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
    let history = use_navigator().unwrap();
    let route: T = use_route().unwrap();
    let active = &route == to;
    let href = to.to_owned().to_path();

    let on_navigate = {
        let history = history.clone();
        let target = to.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            history.push(&target);
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
