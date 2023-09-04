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
        <Title text="🌞 Kyle Dewy 🌞"/>

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
                    <Route path="/projects" view=|| view! { <Projects/> }/>
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
        String::from("hi welcome to my wasm site"),
    ];
    view! {
            <TypingContainer messages=msgs/>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div class="text-blob">
            <p>"I am a developer, mainly focused on writing smart-contracts since 2017. My preferred languages are Solidity, Huff, and Rust."</p>
            <p>"I studied Psychology and Computer Science at the University of British Columbia in Vancouver, before moving to Switzerland to work on smart-contracts full-time."</p>
            <p>"I do consulting and development work so feel free to reach out if you have a project you would like help with."</p>
        </div>
    }
}

#[component]
fn Projects() -> impl IntoView {
    view! {
        <div>
                <ul><strong>2022-present </strong>
                <li> I worked on the smart-contracts for the V2 release for <a href="https://nexusmutual.io/" target="_blank">Nexus Mutual</a> 
                .I was in charge of writing the coverage and staking NFTs as well as working on the new staking system, which utilizes a type of concentrated liquidity to allow stakers to provide coverage for defi protocols at their prefered price-point.
                </li>
                <li> I got interested in <a href="https://huff.sh/" target="_blank">Huff</a> and wrote an efficient implementation of the <a href="https://github.com/kyledewy/quicksort-huff"> quicksort algorithm </a></li>
                <li> To deepen my understanding of low level evm programming I rewrote <a href="https://github.com/kyledewy/weiroll-huff"> weiroll </a> in Huff. This reduced the gas costs by around 50%. </li>

                <li> I started building a city-builder strategy game using Bevy, a rust based game engine. The idea of the game is to terraform planets by altering the weather systems to make it more liveable. </li>
                
                </ul>

                <ul><strong> 2021-2022</strong> 
                    <li> I helped a friend start a defi company called  <a href="https://www.enso.finance/" target="_blank">EnsoFinance</a> with the goal of aggregating multiple defi interactions into a singl interface. We wanted to give users a way to interact with the many different protocols in a single transaction. We initially used tokenized vaults, but to increase gas efficiency and security we instead used the Weiroll VM to aggregate contract calls. This allows users to keep their tokens and ETH seperated and allows for simpler integrations </li>
               </ul> 
               <ul><strong> 2019-2021 </strong> 
                <li> I joined <a href="https://www.seba.swiss/" target="_blank">Seba Bank</a>, where we developed a chain analysis tool to comply with AML laws. Following this we setup the cold storage system. To do this we developed a custom cli using a stripped down image of Debian. The system utilized multisigs for Bitcoin and Ethereum and signed the transactions completely offline for maximum security. </li>

                <li> Me and a friend launched an NFT named <a href="https://opensea.io/collection/galaxia"> Galaxia </a>. Each NFT was a planetary body in our solar system.</li>

                </ul>
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
