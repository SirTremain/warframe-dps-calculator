use crate::Route;
use dioxus::prelude::*;

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            class: "mt-[50px]",

            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            Link {
                to: Route::Blog { id: id - 1 },
                class: "text-white mt-[50px]",
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                class: "text-white mt-[50px]",
                "Next"
            }
        }
    }
}
