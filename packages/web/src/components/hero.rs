use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "flex flex-col justify-center items-center",
            img { src: HEADER_SVG, class: "max-w-[1200px]" }
            div { class: "w-[400px] text-left text-xl text-white flex flex-col",
                a { href: "https://dioxuslabs.com/learn/0.7/", class: "text-white no-underline my-2.5 border border-white rounded-[5px] p-2.5 hover:bg-[#1f1f1f] hover:cursor-pointer", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", class: "text-white no-underline my-2.5 border border-white rounded-[5px] p-2.5 hover:bg-[#1f1f1f] hover:cursor-pointer", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", class: "text-white no-underline my-2.5 border border-white rounded-[5px] p-2.5 hover:bg-[#1f1f1f] hover:cursor-pointer", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", class: "text-white no-underline my-2.5 border border-white rounded-[5px] p-2.5 hover:bg-[#1f1f1f] hover:cursor-pointer", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", class: "text-white no-underline my-2.5 border border-white rounded-[5px] p-2.5 hover:bg-[#1f1f1f] hover:cursor-pointer", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", class: "text-white no-underline my-2.5 border border-white rounded-[5px] p-2.5 hover:bg-[#1f1f1f] hover:cursor-pointer", "👋 Community Discord" }
            }
        }
    }
}
