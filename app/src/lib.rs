use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
pub mod error_template;

use podman_api_stubs::models::ListPodsReport;
use podman_api_stubs::models;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/mcpanel.css"/>
        <Html attr:data-theme="forest"/>

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

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button class="btn btn-primary" on:click=on_click>
            "Click Me: "
            {count}
        </button>
        <NewPodButton/>
        <RunningPods/>
    }
}

#[component]
fn NewPodButton() -> impl IntoView {
    let dialogref = create_node_ref::<html::Dialog>();

    let on_click = move |_| {
        let node = dialogref.get().unwrap();
        node.show_modal();
    };

    let on_submit = move |e: ev::SubmitEvent| {
        // stop the page from reloading!
        e.prevent_default();
    };

    let new_pod = create_server_action::<NewPodFn>();

    view! {
        <button class="btn" on:click=on_click>
            "New Pod"
        </button>
        <dialog ref=dialogref>
            <ActionForm action=new_pod>
                <div class="flex">
                    <h1 class="grow text-3xl">
                        New Pod
                    </h1>
                    <button type="button" on:click=move |_| dialogref.get().unwrap().close()>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M6 18L18 6M6 6l12 12"
                            ></path>
                        </svg>
                    </button>
                </div>
                <div>
                    <label>
                        Pod name
                        <input type="text" name="pod_name"/>
                    </label>
                </div>
                <div>
                    <button type="submit">
                        Create pod
                    </button>
                </div>
            </ActionForm>
        </dialog>
    }
}

#[server(NewPodFn, "/api", "Url", "pods/new")]
async fn new_pod(pod_name: String) -> Result<String, ServerFnError> {
    use podman_api::opts::PodCreateOpts;
    use podman_api::Podman;
    let podman = Podman::unix("/run/user/1000/podman/podman.sock");

    let pod = podman
        .pods()
        .create(&PodCreateOpts::builder().name(pod_name).build())
        .await?;

    let info = pod.inspect().await?;

    Ok(format!("{:?}", info))
}

#[component]
fn RunningPods() -> impl IntoView {
    let pods = create_resource(
        || (),
        |_| async move {
            let x = get_running_pods().await;

            //dbg!(&x);

            x
        },
    );

    // https://github.com/leptos-rs/leptos/discussions/1565#discussioncomment-6794893

    //let pods = move || match pods.get() {
    //    Some(Ok(p)) => Ok(p),
    //    None => Err("Failed to get pods, awaiting the Future returned None".to_string()),
    //    Some(Err(e)) => Err(e.to_string()),
    //};

    //dbg!(&pods());

    //view! {
    //    <h1>"Running Pods"</h1>

    //    <Suspense fallback=|| {
    //        view! { <p>"Loading..."</p> }
    //    }>
    //        <ErrorBoundary fallback=|e| {
    //            view! {
    //                <p>"Error: "</p>
    //                <p>{format!("{:?}", e)}</p>
    //            }
    //        }>

    //            <For
    //                each=pods()
    //                key=|pod| pod.clone()
    //                children=|pod| {
    //                    view! {
    //                            <p class="border-dashed border-4 border-sky-200">{pod}
    //                            </p>
    //                    }
    //                }
    //            />
    //

    //        </ErrorBoundary>
    //    </Suspense>
    //}
    //

    view! {
        <h1>"Running Pods"</h1>
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            <ErrorBoundary fallback=move |e| {
                view! {
                    <p>"Error: "</p>
                    <p>{move || format!("{e:#?}")}</p>
                }
            }>

                <ul>
                    {move || {
                        match pods.get() {
                            Some(Ok(v_pods)) => {
                                v_pods
                                    .iter()
                                    .map(|pod| {
                                        view! {
                                            <li class="border-dashed border-2 border-indigo-500 hover:brightness-50">
                                                <ListPodsReportComponent pod=pod.clone()/>
                                            </li>
                                        }
                                    })
                                    .collect_view()
                            }
                            Some(err) => format!("{err:#?}").into_view(),
                            None => {
                                "get_running_pods serverfn future hasn't resolved yet".into_view()
                            }
                        }
                    }}

                </ul>

            </ErrorBoundary>
        </Transition>
    }
}

#[component]
fn ListPodsReportComponent(pod: ListPodsReport) -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <div class="border-dotted border-2">
                <div class="text-lg">
                    cgroup
                </div>
                <div>{pod.cgroup}</div>
            </div>
            <div class="border-dotted border-2">
                <div class="text-lg">
                    containers
                </div>
                <div>{format!("{:#?}", pod.containers)}</div>
            </div>
            <div class="border-dotted border-2">
                <div class="text-lg">
                    created
                </div>
                <div>{format!("{:#?}", pod.created)}</div>
            </div>
            <div class="border-dotted border-2">
                <div class="text-lg">
                    id
                </div>
                <div>{pod.id}</div>
            </div>
            <div class="border-dotted border-2">
                <div class="text-lg">
                    infra_id
                </div>
                <div>{pod.infra_id}</div>
            </div>
            <div class="border-dotted border-2">
                <div class="text-lg">
                labels
                </div>
                <div>{format!("{:#?}", pod.labels)}</div>
            </div>
            <div class="border-dotted border-2">
                <div class="text-lg">
                    name
                </div>
                <div>{pod.name}</div>
            </div>
            <div class="border-dotted border-2">
                <div class="text-lg">
                    namespace
                </div>
                <div>{pod.namespace}</div>
            </div>
            <div class="border-dotted border-2">
                <div class="text-lg">
                    networks
                </div>
                <div>{format!("{:#?}", pod.networks)}</div>
            </div>
            <div class="border-dotted border-2">
                <div class="text-lg">
                    status
                </div>
                <div>{pod.status}</div>
            </div>

        </div>
    }
}

#[server(GetRunningPods, "/api", "GetJson", "pods")]
async fn get_running_pods() -> Result<Vec<ListPodsReport>, ServerFnError> {
    use podman_api::opts::PodListOpts;
    use podman_api::Podman;
    let podman = Podman::unix("/run/user/1000/podman/podman.sock");

    let pods = podman.pods().list(&PodListOpts::default()).await?;


    Ok(pods)
}
