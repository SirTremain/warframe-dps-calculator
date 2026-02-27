use crate::components::Template;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        // Components can be used inside rsx just like HTML elements. Props are passed as attributes.
        Template {
            title: "Warframe DPS Calculator".to_string(),
            body: "Welcome! This app is under construction.".to_string(),
        }
    }
}
