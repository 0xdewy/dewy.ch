use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/kyledewy.css"/>
        <Link rel="icon" type_="image/x-icon" href="favicon.ico"/>
        <Title text="kyle dewy"/>

        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let msgs = vec![
        String::from("hi welcome to my website"),
        String::from("my name is kyle, i am a programmer"),
    ];
    view! { cx,
        <NavBar/>
        <AppLayout>
            <TypingContainer messages=msgs/>
        </AppLayout>
        <SocialMedia/>
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

