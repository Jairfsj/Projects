use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <main class="flex flex-col items-center justify-center flex-grow bg-gradient-to-r from-gray-100 to-gray-300 p-8">
            <img class="logo w-32 h-32 mb-8 hover:scale-105 transition-transform duration-300" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1 class="text-4xl font-bold text-gray-800 mb-6">
                { "Bem Vindo! Esta Página é Sobre Rust!" }
            </h1>
            <p class="text-lg text-gray-600 mb-8 text-center">
                { "Este é o meu portfólio pessoal, construído com Yew e Rust." }
            </p>
            <Link<Route> to={Route::Contatos} classes="btn-secondary">
                { "Entre em contato" }
            </Link<Route>>
        </main>
    }
}
