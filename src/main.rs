use sycamore::prelude::*;

#[component]
fn Counter() -> View {
    let count = create_signal(0);

    view! {
        div {
            h1 { "Counter" }
            p { (count) }
            button(on_click=move |_| count.set(count.get() + 1)) { "Increment" }
        }
    }
}

#[component]
fn App() -> View {
    view! {
        div {
            h1 { "Hello, Sycamore!" }
            p { "This is a simple Sycamore application." }
        }

        Counter()
    }
}

fn main() {
    sycamore::render(App)
}
