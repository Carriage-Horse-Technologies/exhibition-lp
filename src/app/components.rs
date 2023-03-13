use std::fmt::Display;

use wasm_bindgen::JsValue;
use web_sys::HtmlElement;
use yew::NodeRef;

pub(crate) mod balloon;
pub(crate) mod chat_text_field;
pub(crate) mod enter_button;
pub(crate) mod header;
pub(crate) mod input_user_name;
pub(crate) mod modals;
pub(crate) mod myself;
pub(crate) mod other_character;
pub(crate) mod product;
pub(crate) mod product_list;

fn move_node<T>(node: &NodeRef, x: &T, y: &T, duration_ms: u32) -> Result<(), JsValue>
where
    T: Display,
{
    let style = node.cast::<HtmlElement>().unwrap().style();
    style.set_property("transform", &format!("translate({}px, {}px)", x, y))?;
    style.set_property("transition-duration", &format!("{}ms", duration_ms))
}
