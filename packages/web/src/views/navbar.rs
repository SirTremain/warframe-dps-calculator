use crate::Route;
use dioxus::prelude::*;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "flex flex-row",
            Link {
                to: Route::Home {},
                class: "text-white mr-5 no-underline transition-colors duration-200 ease-in-out hover:cursor-pointer hover:text-[#91a4d2]",
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                class: "text-white mr-5 no-underline transition-colors duration-200 ease-in-out hover:cursor-pointer hover:text-[#91a4d2]",
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
