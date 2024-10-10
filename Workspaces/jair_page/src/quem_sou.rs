use yew::prelude::*;

#[function_component(QuemSou)]
pub fn quem_sou() -> Html {
    html! {
        <main class="flex flex-col items-center justify-center flex-grow bg-gradient-to-r from-gray-100 to-gray-300 p-8">
            <h1 class="text-4xl font-bold text-gray-800 mb-6">
                { "Sobre Mim" }
            </h1>
            <p class="text-lg text-gray-600 mb-8 text-center">
                { "Eu sou um desenvolvedor especializado em Rust, com experiência em diversas áreas de programação." }
            </p>
        </main>
    }
}
