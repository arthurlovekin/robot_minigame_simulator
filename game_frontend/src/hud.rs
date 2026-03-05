#![allow(clippy::wildcard_imports)] // Leptos 0.7 view! macro requires many traits in scope.

use leptos::prelude::*;

#[component]
pub fn Hud(score: RwSignal<f32>, terminated: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="hud">
            <div>"Score: " {move || format!("{:.0}", score.get())}</div>
            {move || terminated.get().then(|| view! {
                <div style="margin-top:8px;color:#f88">"GAME OVER \u{2014} Press R to restart"</div>
            })}
        </div>
    }
}
