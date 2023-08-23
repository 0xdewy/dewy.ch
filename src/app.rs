use crate::error_template::{AppError, ErrorTemplate};
use frankenstein::TelegramApi;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/kyledewy.css"/>
        <Link rel="icon" type_="image/x-icon" href="favicon.svg"/>
        <Title text="kyle dewy 🌞"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
            <NavBar/>
            <AppLayout>
                <Routes>
                    <Route path="" view=|| view! { <HomePage/> }/>
                    <Route path="/contact" view=|| view! { <Contact/> }/>
                    <Route path="/about" view=|| view! { <About/> }/>
                </Routes>
            </AppLayout>
            <SocialMedia/>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // TODO: iterate through the messages
    let msgs = vec![
        String::from("hi welcome to my website"),
        String::from("my name is kyle, i am a programmer"),
    ];
    view! {
            <TypingContainer messages=msgs/>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div>
            <p>"have been developing blockchain software since 2016"</p>
        </div>
    }
}

#[server(SendMessage, "/api")]
pub async fn send_message(
    name: String,
    email: String,
    message: String,
) -> Result<(), ServerFnError> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN")?;
    let chat_id = std::env::var("TELEGRAM_CHAT_ID")?;
    let api = frankenstein::Api::new(&token);
    let send_message_params = frankenstein::SendMessageParams::builder()
        .chat_id(chat_id)
        .text(format!(
            "name: {}\nemail: {}\nmessage: {}",
            name, email, message
        ))
        .allow_sending_without_reply(true)
        .build();
    println!("sending message {send_message_params:?}");
    let res = api.send_message(&send_message_params)?;
    Ok(())
}

#[component]
fn Contact() -> impl IntoView {
    let send_message = create_server_action::<SendMessage>();
    view! {
    <ActionForm class="vertical-form" action=send_message>
        <input type="text" name="name" placeholder="Your Name" required/>
        <input type="email" name="email" placeholder="Your Email" required/>
        <textarea name="message" placeholder="Your Message" required></textarea>
        <button type="submit">Send Message</button>
    </ActionForm>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <div class="nav-bar">
                <a class="nav-item" href="/">"home"</a>
                <a class="nav-item" href="/projects">"projects"</a>
                <a class="nav-item" href="/contact">"contact"</a>
                <a class="nav-item" href="/about">"about"</a>
        </div>
    }
}

#[component]
fn SocialMedia() -> impl IntoView {
    view! {
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
fn AppLayout(children: Children) -> impl IntoView {
    view! {
        <div class="app">

        <div class="content">
            {children()}

        </div>
    </div>
    }
}

#[component]
fn TypingContainer(messages: Vec<String>) -> impl IntoView {
    let (index, _set_index) = create_signal(0);

    /*
    let msg_len  = messages.len();
    */
    let current_message = Signal::derive(move || messages[index.get() as usize].clone());

    view! {
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
