use yew::{function_component, html, Properties, Callback};

#[derive(Properties, Clone, PartialEq)]
pub struct CyberSliderProps {
    pub checked: bool,
    pub onchange: Callback<bool>,
    pub disabled: Option<bool>,
    pub id: Option<String>,
    pub label: Option<String>,
}

#[function_component(CyberSlider)]
pub fn cyber_slider(props: &CyberSliderProps) -> Html {
    let CyberSliderProps { checked, onchange, disabled, id, label } = props;
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

    let slider_id = id.clone().unwrap_or_else(|| "cyber-slider".to_string());
    
    html! {
        <div class="cyber-slider-container">
            <div 
                class={format!("cyber-slider {}{}", 
                    if *checked { "active" } else { "inactive" },
                    if disabled { " disabled" } else { "" }
                )}
                onclick={onclick}
                role="switch"
                aria-checked={checked.to_string()}
                aria-disabled={disabled.to_string()}
                tabindex={if disabled { "-1" } else { "0" }}
            >
                // CYBER SLIDER TRACK - EPIC GLOW!!!
                <div class="slider-track">
                    <div class="track-glow"></div>
                    <div class="track-inner"></div>
                </div>
                
                // SLIDING CORE BUTTON - MEGA ANIMATIONS!!!
                <div class="slider-core">
                    <div class="core-glow"></div>
                    <div class="core-inner">
                        <div class="core-pulse"></div>
                    </div>
                </div>
            </div>
            
            if let Some(label_text) = label {
                <label 
                    class={format!("cyber-slider-label{}", if disabled { " disabled" } else { "" })}
                    for={slider_id}
                >
                    {label_text}
                </label>
            }
        </div>
    }
}