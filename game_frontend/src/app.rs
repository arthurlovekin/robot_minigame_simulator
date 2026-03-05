#![allow(clippy::wildcard_imports)] // Leptos 0.7 view! macro requires many traits in scope.

use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use lunar_lander::{LunarLanderConfig, LunarLanderEnv};

use crate::game_loop;
use crate::hud::Hud;
use crate::input;
use crate::input_state::KeysState;

#[component]
pub fn App() -> impl IntoView {
    let svg_html = RwSignal::new(String::new());
    let score = RwSignal::new(0.0_f32);
    let terminated = RwSignal::new(false);

    let mut env = LunarLanderEnv::new(LunarLanderConfig::default());
    env.reset();
    let env = Rc::new(RefCell::new(env));
    let keys: Rc<RefCell<KeysState>> = Rc::new(RefCell::new(KeysState::default()));

    let document = web_sys::window().expect("window").document().expect("document");
    input::register_listeners(&document, keys.clone());

    let window = web_sys::window().expect("window");
    game_loop::start(window, env, keys, svg_html, score, terminated);

    view! {
        <div>
            <div inner_html=move || svg_html.get()/>
            <Hud score=score terminated=terminated/>
        </div>
    }
}
