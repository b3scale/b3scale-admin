use web_sys::HtmlElement;
use yew::{function_component, html, use_state, Callback, Properties, Html, MouseEvent, TargetCast};

#[derive(Properties, Clone, PartialEq)]
pub struct CyberDropdownProps {
    pub value: String,
    pub options: Vec<(String, String)>, // (value, label)
    pub on_change: Callback<String>,
    pub disabled: bool,
}

#[function_component(CyberDropdown)]
pub fn cyber_dropdown(props: &CyberDropdownProps) -> Html {
    let CyberDropdownProps { value, options, on_change, disabled } = props;
    let is_open = use_state(|| false);
    
    let toggle_dropdown = {
        let is_open = is_open.clone();
        let disabled = *disabled;
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if !disabled {
                is_open.set(!*is_open);
            }
        })
    };
    
    let select_option = {
        let is_open = is_open.clone();
        let on_change = on_change.clone();
        Callback::from(move |value: String| {
            is_open.set(false);
            on_change.emit(value);
        })
    };
    
    let current_label = options.iter()
        .find(|(v, _)| v == value)
        .map(|(_, l)| l.as_str())
        .unwrap_or("");
    
    html! {
        <div class="cyber-dropdown-container" style="position: relative;">
            <div 
                class={if *disabled { "cyber-dropdown disabled" } else { "cyber-dropdown" }}
                onclick={toggle_dropdown}
                style="
                    background: linear-gradient(135deg, rgba(255, 0, 127, 0.3), rgba(0, 255, 255, 0.3), rgba(191, 0, 255, 0.3));
                    border: 3px solid transparent;
                    border-image: linear-gradient(135deg, #ff007f, #00ffff, #bf00ff) 1;
                    padding: 12px 16px;
                    cursor: pointer;
                    color: #ff99cc;
                    text-shadow: 0 0 10px rgba(255, 0, 127, 0.8), 0 0 20px rgba(0, 255, 255, 0.6);
                    font-weight: 700;
                    letter-spacing: 1px;
                    text-transform: uppercase;
                    box-shadow: 0 0 20px rgba(255, 0, 127, 0.6), 0 0 30px rgba(0, 255, 255, 0.6);
                    border-radius: 8px;
                    position: relative;
                    user-select: none;
                "
            >
                <div style="display: flex; justify-content: space-between; align-items: center;">
                    <span>{current_label}</span>
                    <span style="margin-left: 10px; transform: rotate(90deg); font-size: 1.2em;">{"▸"}</span>
                </div>
            </div>
            
            if *is_open {
                <div class="cyber-dropdown-options" 
                    style="
                        position: absolute;
                        top: 100%;
                        left: 0;
                        right: 0;
                        margin-top: 5px;
                        background: linear-gradient(135deg, rgba(0, 0, 0, 0.95), rgba(255, 0, 127, 0.2), rgba(0, 0, 0, 0.95));
                        border: 3px solid transparent;
                        border-image: linear-gradient(135deg, #ff007f, #00ffff, #bf00ff) 1;
                        border-radius: 8px;
                        overflow: hidden;
                        z-index: 1000;
                        box-shadow: 0 0 30px rgba(255, 0, 127, 0.8), 0 0 40px rgba(0, 255, 255, 0.8);
                    "
                >
                    {for options.iter().map(|(val, label)| {
                        let value_clone = val.clone();
                        let is_selected = val == value;
                        let onclick = {
                            let select_option = select_option.clone();
                            Callback::from(move |e: MouseEvent| {
                                e.prevent_default();
                                select_option.emit(value_clone.clone());
                            })
                        };
                        
                        html! {
                            <div 
                                class="cyber-dropdown-option"
                                onclick={onclick}
                                style={format!("
                                    padding: 12px 16px;
                                    cursor: pointer;
                                    color: {};
                                    text-shadow: 0 0 10px rgba(255, 0, 127, 0.6), 0 0 15px rgba(0, 255, 255, 0.4);
                                    font-weight: 600;
                                    letter-spacing: 1px;
                                    text-transform: uppercase;
                                    background: {};
                                    transition: all 0.3s ease;
                                ",
                                    if is_selected { "#ffff00" } else { "#ff99cc" },
                                    if is_selected { "rgba(255, 0, 127, 0.3)" } else { "transparent" }
                                )}
                                onmouseover={Callback::from(|e: MouseEvent| {
                                    if let Some(target) = e.target_dyn_into::<HtmlElement>() {
                                        target.set_attribute("style", "
                                            padding: 12px 16px;
                                            cursor: pointer;
                                            color: #00ffff;
                                            text-shadow: 0 0 15px rgba(0, 255, 255, 0.8), 0 0 25px rgba(255, 0, 127, 0.6);
                                            font-weight: 600;
                                            letter-spacing: 1px;
                                            text-transform: uppercase;
                                            background: linear-gradient(90deg, rgba(255, 0, 127, 0.3), rgba(0, 255, 255, 0.3));
                                            transition: all 0.3s ease;
                                        ").unwrap();
                                    }
                                })}
                                onmouseout={Callback::from(move |e: MouseEvent| {
                                    if let Some(target) = e.target_dyn_into::<HtmlElement>() {
                                        let style = if is_selected {
                                            "padding: 12px 16px; cursor: pointer; color: #ffff00; text-shadow: 0 0 10px rgba(255, 0, 127, 0.6), 0 0 15px rgba(0, 255, 255, 0.4); font-weight: 600; letter-spacing: 1px; text-transform: uppercase; background: rgba(255, 0, 127, 0.3); transition: all 0.3s ease;"
                                        } else {
                                            "padding: 12px 16px; cursor: pointer; color: #ff99cc; text-shadow: 0 0 10px rgba(255, 0, 127, 0.6), 0 0 15px rgba(0, 255, 255, 0.4); font-weight: 600; letter-spacing: 1px; text-transform: uppercase; background: transparent; transition: all 0.3s ease;"
                                        };
                                        target.set_attribute("style", style).unwrap();
                                    }
                                })}
                            >
                                {label}
                            </div>
                        }
                    })}
                </div>
            }
        </div>
    }
}