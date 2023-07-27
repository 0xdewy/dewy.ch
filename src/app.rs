use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use frankenstein::TelegramApi;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/kyledewy.css"/>
        <Link rel="icon" type_="image/x-icon" href="favicon.svg"/>
        <Title text="kyle dewy 🌞"/>

        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
            <NavBar/>
            <AppLayout>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/contact" view=|cx| view! { cx, <Contact/> }/>
                </Routes>
            </AppLayout>
            <SocialMedia/>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // TODO: iterate through the messages
    let msgs = vec![
        String::from("hi welcome to my website"),
        String::from("my name is kyle, i am a programmer"),
    ];
    view! { cx,
            <TypingContainer messages=msgs/>
    }
}

#[server(SendMessage, "/api")]
pub async fn send_message(cx: Scope, name: String, email: String, message: String) -> Result<(), ServerFnError> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN")?;
    let chat_id = std::env::var("TELEGRAM_CHAT_ID")?;
    let api = frankenstein::Api::new(&token);
    let send_message_params = frankenstein::SendMessageParams::builder()
    .chat_id(chat_id)
    .text(format!("name: {}\nemail: {}\nmessage: {}", name, email, message))
    .allow_sending_without_reply(true)
    .build();
    println!("sending message {send_message_params:?}");
    let res = api.send_message(&send_message_params)?;
    Ok(())
}


#[component]
fn Contact(cx: Scope) -> impl IntoView {
    let send_message = create_server_action::<SendMessage>(cx);
    view! { cx,
    <ActionForm class="vertical-form" action=send_message>
        <input type="text" name="name" placeholder="Your Name" required/>
        <input type="email" name="email" placeholder="Your Email" required/>
        <textarea name="message" placeholder="Your Message" required></textarea>
        <button type="submit">Send Message</button>
    </ActionForm>
    }
}

#[component]
fn NavBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="nav-bar">
                <a class="nav-item" href="/">"home"</a>
                <a class="nav-item" href="/projects">"projects"</a>
                <a class="nav-item" href="/contact">"contact"</a>
                <a class="nav-item" href="/about">"about"</a>
        </div>
    }
}

#[component]
fn SocialMedia(cx: Scope) -> impl IntoView {
    view! { cx,
            <div class="social-media">
                <a href="https://twitter.com/kyle_dewy" target="_blank">
                    <div class="twitter" ></div>
                </a>
                <a href="https://github.com/kyledewy" target="_blank">
                    <div class="github" />
                </a>
            </div>
    }

}

#[component]
fn AppLayout(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="app">
            
        <div class="content">
            {children(cx)}
            
        </div>
    </div>
    }
}

#[component]
fn TypingContainer(cx: Scope, messages: Vec<String>) -> impl IntoView {
    let (index, _set_index) = create_signal(cx, 0);

    /*
    let msg_len  = messages.len();
    */
    let current_message = Signal::derive(cx, move || messages[index.get() as usize].clone());


    view! { cx,
            <div class="typing-container">

            <div class="typed-out"
                /*
                 on:animationend=move |_| {
                    let curr_index = index.get();
                    set_index.set((curr_index + 1) % msg_len);
                 }
                 */
            >
                {current_message}
            </div>
        </div>
    }
}

