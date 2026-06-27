use dioxus::prelude::*;
use latent_calculator::{Calculator, ParseError};

const USER_AVATAR: Asset = asset!("/assets/profile.jpg");
const CHAT_LIST_ID: &str = "chat-messages";
const BOT_SEED_TEXT: &str = "Hi! I'm LatCal \u{1f9ee} \u{2014} ask me math in plain words, e.g. \u{201c}10$ discount 2%\u{201d}.";
const INPUT_PLACEHOLDER: &str = "ask a math question in plain words\u{2026}";

/// One line in the conversation. `from_user` decides bubble side + avatar.
#[derive(Clone, PartialEq)]
struct Message {
    id: usize,
    from_user: bool,
    text: String,
}

fn bot_reply(input: &str) -> String {
    match Calculator::parse(input) {
        Ok(answer) => answer.to_sentence(),
        Err(ParseError::NotMath) => "that doesn't look like a math question".to_string(),
        Err(ParseError::Unknown) => "sorry, I could not understand that".to_string(),
    }
}

/// Stick the transcript to the newest message. Runs after the DOM commit, so
/// the freshly pushed bubble is already laid out when we read scrollHeight.
fn scroll_chat_to_bottom() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    if let Some(el) = document.get_element_by_id(CHAT_LIST_ID) {
        el.set_scroll_top(el.scroll_height());
    }
}

/// Chat page — messenger-style UI powered by LatCal.
/// Fullscreen standalone layout (Nav + this). State lives in signals; the
/// bot reply is computed synchronously from the user's message on send.
#[component]
pub fn Chat() -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    let mut messages = use_signal(Vec::<Message>::new);
    let mut input = use_signal(String::new);
    let mut next_id: Signal<usize> = use_signal(|| 1);

    // Seed the conversation once.
    use_hook(|| {
        messages.write().push(Message {
            id: 0,
            from_user: false,
            text: BOT_SEED_TEXT.to_string(),
        });
    });

    // Derived from `input` directly — no extra state to keep in sync.
    let can_send = use_memo(move || !input.read().trim().is_empty());

    // Auto-scroll whenever the transcript changes (incl. the seed on mount).
    use_effect(move || {
        let _len = messages.read().len();
        scroll_chat_to_bottom();
    });

    let mut send = move |_| {
        let text = input.read().trim().to_string();
        if text.is_empty() {
            return;
        }
        let user_id = *next_id.read();
        *next_id.write() += 1;
        messages.write().push(Message {
            id: user_id,
            from_user: true,
            text: text.clone(),
        });

        let bot_id = *next_id.read();
        *next_id.write() += 1;
        let reply = bot_reply(&text);
        messages.write().push(Message {
            id: bot_id,
            from_user: false,
            text: reply,
        });

        input.write().clear();
    };

    rsx! {
        section {
            class: if dark_mode() { "chat-app" } else { "chat-app light-mode" },

            div {
                id: CHAT_LIST_ID,
                class: "chat-messages",
                for msg in messages.read().iter() {
                    div {
                        key: "{msg.id}",
                        class: if msg.from_user { "chat-row user" } else { "chat-row bot" },
                        if msg.from_user {
                            img { class: "chat-avatar", src: USER_AVATAR, alt: "you" }
                        } else {
                            span { class: "chat-avatar emoji", "🧮" }
                        }
                        div { class: "chat-bubble", "{msg.text}" }
                    }
                }
            }

            form {
                class: "chat-input-bar",
                onsubmit: move |e| {
                    e.prevent_default();
                    send(());
                },
                input {
                    class: "chat-input",
                    value: "{input}",
                    oninput: move |e| input.set(e.value()),
                    placeholder: INPUT_PLACEHOLDER,
                    autocomplete: "off",
                }
                button {
                    class: "chat-send",
                    r#type: "submit",
                    disabled: !can_send(),
                    "Send"
                }
            }
        }
    }
}
