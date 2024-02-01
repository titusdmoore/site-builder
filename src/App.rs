use leptos::html::Canvas;
use leptos::logging::log;
use leptos::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
enum WorkspaceState {
    Preview,
    Build,
}

impl Display for WorkspaceState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspaceState::Preview => write!(f, "Preview"),
            WorkspaceState::Build => write!(f, "Build"),
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (workspace_state, set_workspace_state) = create_signal(WorkspaceState::Build);
    let canvas = create_node_ref::<Canvas>();

    view! {
        <main class="w-full h-full grid grid-cols-12 min-h-screen">
            <div class="col-span-9 h-full">
                <header>
                    <h1 class="text-4xl">"Page Builder"</h1>
                </header>
                <section class="h-full p-4">
                    <header class="flex justify-between mb-4">
                        <h2 class="text-2xl">{workspace_state.get().to_string()}" Page"</h2>
                    </header>
                    <div class="h-full">
                        <canvas class="w-full h-full bg-gray-200 rounded-md" id="canvas"/>
                    </div>
                </section>
            </div>
            <aside class="col-span-3">
                <div class="p-4">
                    <h2 class="text-2xl mb-8">"Sidebar"</h2>
                    <div class="p-2 bg-blue-500 w-fit" draggable="true" on:drag=move |e| {
                        e.prevent_default();
                        log!("dragging");
                    }>Testing Element</div>
                </div>
            </aside>
        </main>
    }
}
