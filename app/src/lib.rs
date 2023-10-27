use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/mcpanel.css"/>

        // sets the document title
        <Title text="Mcpanel"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let np = move |_| {
        spawn_local(async {
            new_pod().await;
        })
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button class="btn btn-primary" on:click=on_click>"Click Me: " {count}</button>
        <button class="btn btn-secondary" on:click=np>"Create pod"</button>
    }
}

#[server]
async fn new_pod() -> Result<(), ServerFnError> {
    use podman_api::opts::PodCreateOpts;
    use podman_api::Podman;
    let podman = Podman::unix("/run/user/1000/podman/podman.sock");

    match podman
        .pods()
        .create(
            &PodCreateOpts::builder()
                .name("pod-created-from-webapp")
                .build(),
        )
        .await
    {
        Ok(pod) => {
            dbg!(&pod);
        }
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}
