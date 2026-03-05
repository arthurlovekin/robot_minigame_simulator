use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::{GetUntracked, RwSignal, Set};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Window;

use display::RenderConfig;
use lunar_lander::LunarLanderEnv;
use simulator_types::Vec2;

use crate::input_state::KeysState;

const LANDER_BODY_IDX: usize = 1;
const SCORE_ALTITUDE_SCALE: f32 = 1.0;

pub fn start(
    window: Window,
    env: Rc<RefCell<LunarLanderEnv>>,
    keys: Rc<RefCell<KeysState>>,
    svg_html: RwSignal<String>,
    score: RwSignal<f32>,
    terminated: RwSignal<bool>,
) {
    // Shared recursive closure slot.
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        // Handle restart before anything else.
        let restart = {
            let mut k = keys.borrow_mut();
            let r = k.restart;
            k.restart = false;
            r
        };
        if restart {
            env.borrow_mut().reset();
            score.set(0.0);
            terminated.set(false);
        }

        if !terminated.get_untracked() {
            let action = keys.borrow().to_action();
            let (obs, term, state) = env.borrow_mut().step(action);

            // Accumulate score: reward altitude preservation.
            let altitude = obs[3];
            score.set(score.get_untracked() + altitude * SCORE_ALTITUDE_SCALE);

            // Camera follows the lander.
            let cam = state
                .bodies
                .get(LANDER_BODY_IDX)
                .map(|b| b.position)
                .unwrap_or(Vec2 { x: 0.0, y: 5.0 });

            let config = RenderConfig { camera_centre: cam, ..RenderConfig::default() };
            svg_html.set(display::render(&state, &config));

            if term {
                terminated.set(true);
            }
        }

        // Schedule next frame.
        window
            .request_animation_frame(
                f.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
            )
            .expect("request_animation_frame");
    }));

    web_sys::window()
        .expect("window")
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("request_animation_frame");
}
