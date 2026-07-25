use kode_leptos::CodeEditor;
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <main style="display: flex; height: 100vh;">
                <CodeEditor />
                <CodeEditor />
            </main>
        }
    })
}
