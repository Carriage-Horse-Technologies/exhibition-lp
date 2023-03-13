use std::{cell::RefCell, rc::Rc};

use web_sys::{HtmlTextAreaElement, WebSocket};
use yew::prelude::*;
use yew_hooks::use_timeout;
use yewdux::prelude::{use_store, use_store_value};

use crate::{
    app::{
        models::{ChatMessage, LocationType},
        states::{ChatTextFieldState, ChatTextHashState, ChatTextState, Username},
    },
    settings,
};

#[derive(PartialEq, Properties)]
pub(crate) struct ChatTextFieldProps {
    pub(crate) ws: Rc<RefCell<Option<WebSocket>>>,
}

#[function_component]
pub(crate) fn ChatTextField(props: &ChatTextFieldProps) -> Html {
    let ChatTextFieldProps { ws } = props;

    let username = use_store_value::<Username>();
    let node = use_node_ref();
    let (_, chat_text_dispatch) = use_store::<ChatTextHashState>();
    let (_, chat_text_field_onfocus_dispatch) = use_store::<ChatTextFieldState>();

    // 送信5秒後に吹き出しを非表示にするcallback
    let balloon_timeout = {
        let username = username.clone();
        let chat_text_dispatch = chat_text_dispatch.clone();

        use_timeout(
            move || {
                chat_text_dispatch.reduce_mut(|state| {
                    state
                        .hash
                        .insert(username.0.clone(), ChatTextState::default())
                })
            },
            5000,
        )
    };

    let onkeydown = {
        let username = username.clone();
        let node = node.clone();
        let chat_text_dispatch = chat_text_dispatch;
        let balloon_timeout = balloon_timeout;
        let ws = ws.clone();
        Callback::from(move |e: KeyboardEvent| {
            log::debug!(
                "keypress ctrl: {}; enter: {} {} {} meta {}",
                e.ctrl_key(),
                e.key_code(),
                e.code(),
                e.char_code(),
                e.meta_key()
            );

            if (e.ctrl_key() || e.meta_key()) && (e.code() == "Enter" || e.code() == "NumpadEnter")
            {
                let textarea = node.cast::<HtmlTextAreaElement>().unwrap();
                log::debug!("Send chat message. value: {}", textarea.value());
                chat_text_dispatch.reduce_mut(|state| {
                    state.hash.insert(
                        username.0.clone(),
                        ChatTextState {
                            message: textarea.value(),
                            is_display_balloon: true,
                        },
                    );
                });

                // WS
                let chat_message = ChatMessage {
                    action: LocationType::ActionChatMessage,
                    user_id: username.0.clone(),
                    message: textarea.value(),
                };
                if let Err(send_result) = (*ws)
                    .borrow()
                    .clone()
                    .unwrap()
                    .send_with_str(&serde_json::to_string(&chat_message).unwrap())
                {
                    log::error!(
                        "Failed to Websocket send error. {}",
                        send_result.as_string().unwrap_or_default()
                    );
                } else {
                    log::debug!("Success websocket send");
                }
                balloon_timeout.reset();
                textarea.set_value("");
            }
        })
    };

    let onfocusin = {
        let chat_text_field_onfocus_dispatch = chat_text_field_onfocus_dispatch.clone();
        Callback::from(move |_| {
            log::debug!("onfocusin");
            chat_text_field_onfocus_dispatch.reduce(|_| ChatTextFieldState { onfocus: true }.into())
        })
    };

    let onfocusout = {
        let chat_text_field_onfocus_dispatch = chat_text_field_onfocus_dispatch.clone();
        Callback::from(move |_| {
            log::debug!("onfocusout");
            chat_text_field_onfocus_dispatch
                .reduce(|_| ChatTextFieldState { onfocus: false }.into())
        })
    };

    html! {
        <textarea ref={node} onkeydown={onkeydown} onfocusin={onfocusin} onfocusout={onfocusout} name="chat" id="chat" cols="40" rows="3"
            placeholder={"Input text message...\nCtrl+Enter to send"}
            class={classes!("fixed", "rounded-2xl", "bg-dark-primary-deep",
            "bottom-[50px]", "left-[70vw]", "p-2", "z-[930]"
            )}
        />
    }
}
