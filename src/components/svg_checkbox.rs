use yew::{function_component, html, Properties, Callback};

#[derive(Properties, Clone, PartialEq)]
pub struct SvgCheckboxProps {
    pub checked: bool,
    pub onchange: Callback<bool>,
    pub disabled: Option<bool>,
    pub id: Option<String>,
    pub label: Option<String>,
}

#[function_component(SvgCheckbox)]
pub fn svg_checkbox(props: &SvgCheckboxProps) -> Html {
    let SvgCheckboxProps { checked, onchange, disabled, id, label } = props;
    let disabled = disabled.unwrap_or(false);
    
    let onclick = {
        let onchange = onchange.clone();
        let checked = *checked;
        let disabled = disabled;
        Callback::from(move |_| {
            if !disabled {
                onchange.emit(!checked);
            }
        })
    };

    let checkbox_id = id.clone().unwrap_or_else(|| "svg-checkbox".to_string());
    
    html! {
        <div class="svg-checkbox-container">
            <div 
                class={format!("svg-checkbox {}{}", 
                    if *checked { "checked" } else { "" },
                    if disabled { " disabled" } else { "" }
                )}
                onclick={onclick}
                role="checkbox"
                aria-checked={checked.to_string()}
                aria-disabled={disabled.to_string()}
                tabindex={if disabled { "-1" } else { "0" }}
            >
                <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                    // Background circle/square with MEGA VISIBLE BORDER!!!
                    <rect 
                        x="2" 
                        y="2" 
                        width="16" 
                        height="16" 
                        rx="3" 
                        fill={if *checked { "#00ffff" } else { "rgba(0, 255, 255, 0.1)" }}
                        stroke={if *checked { "#00ffff" } else { "#00ffff" }}
                        stroke-width={if *checked { "2" } else { "3" }}
                        class="checkbox-bg"
                    />
                    
                    // CORE GLOW EFFECT - Always visible!!!
                    <rect 
                        x="1" 
                        y="1" 
                        width="18" 
                        height="18" 
                        rx="4" 
                        fill="none"
                        stroke={if *checked { "rgba(0, 255, 255, 0.6)" } else { "rgba(0, 255, 255, 0.8)" }}
                        stroke-width="1"
                        class="checkbox-glow"
                    />
                    // Checkmark
                    if *checked {
                        <path 
                            d="M6 10l3 3 6-6" 
                            stroke="white" 
                            stroke-width="2.5" 
                            stroke-linecap="round" 
                            stroke-linejoin="round"
                            class="checkmark"
                        />
                    }
                </svg>
            </div>
            
            if let Some(label_text) = label {
                <label 
                    class={format!("svg-checkbox-label{}", if disabled { " disabled" } else { "" })}
                    for={checkbox_id}
                >
                    {label_text}
                </label>
            }
        </div>
    }
}