use dioxus::prelude::*;

/// A template component that demonstrates how to build a reusable component with props.
///
/// Components are functions annotated with `#[component]`. Props must be owned types that implement
/// `PartialEq` and `Clone`. Use `ReadOnlySignal` to make props reactive.
#[component]
pub fn Template(
    /// The title text to display in the component.
    title: String,
    /// The body text to display below the title.
    body: String,
) -> Element {
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        div {
            class: "mt-8 p-6 bg-[#1e222d] rounded-lg max-w-xl mx-auto",
            h2 { class: "text-xl font-bold mb-2", "{title}" }
            p { class: "text-gray-300", "{body}" }
        }
    }
}
