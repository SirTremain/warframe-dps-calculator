use dioxus::prelude::*;

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(String::new);

    rsx! {
        div {
            class: "w-[360px] mx-auto mt-[50px] bg-[#1e222d] p-5 rounded-[10px]",
            h4 { class: "mb-[15px]", "ServerFn Echo" }
            input {
                class: "border-0 border-b border-white bg-transparent text-white outline-none block pb-[5px] w-full transition-colors duration-200 ease-in-out focus:border-[#6d85c6]",
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    class: "mt-5 ml-auto",
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

#[post("/api/echo")]
async fn echo_server(input: String) -> Result<String> {
    Ok(input)
}
