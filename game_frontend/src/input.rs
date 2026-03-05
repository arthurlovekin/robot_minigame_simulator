use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, KeyboardEvent};

use crate::input_state::KeysState;

fn set_key(keys: &mut KeysState, code: &str, pressed: bool) {
    match code {
        "Space" | "ArrowUp" => keys.thrust_main = pressed,
        "KeyA" | "ArrowLeft" => keys.thrust_left = pressed,
        "KeyD" | "ArrowRight" => keys.thrust_right = pressed,
        "KeyR" => keys.restart = pressed,
        _ => {}
    }
}

/// Register keydown/keyup listeners on the document.
///
/// The closures are intentionally leaked (`forget`) so they live for the document's lifetime.
pub fn register_listeners(document: &Document, keys: Rc<RefCell<KeysState>>) {
    let keys_down = keys.clone();
    let on_keydown = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
        set_key(&mut keys_down.borrow_mut(), &e.code(), true);
    });
    document
        .add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())
        .expect("add keydown listener");
    on_keydown.forget();

    let on_keyup = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
        set_key(&mut keys.borrow_mut(), &e.code(), false);
    });
    document
        .add_event_listener_with_callback("keyup", on_keyup.as_ref().unchecked_ref())
        .expect("add keyup listener");
    on_keyup.forget();
}
