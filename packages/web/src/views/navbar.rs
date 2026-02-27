use crate::Route;
use dioxus::prelude::*;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
/// This layout component wraps all child routes in a common navbar. The contents of the child
/// routes will be rendered inside the `Outlet` component below.
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
        }

        // The `Outlet` component is used to render the next component inside the layout. It will render
        // the component for the currently active route that is nested under this layout.
        Outlet::<Route> {}
    }
}
