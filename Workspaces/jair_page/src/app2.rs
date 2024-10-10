use yew::prelude::*;

// Componente Navbar
#[function_component(Navbar)]
fn navbar() -> Html {
    html! {
        <nav class="bg-gray-800 p-4 w-full">
            <div class="container mx-auto flex justify-between items-center">
                <div class="text-white text-lg font-semibold">
                    { "Meu Portfólio" }
                </div>
                <ul class="flex space-x-6">
                    <li>
                        <a href="#" class="text-gray-300 hover:text-white">
                            { "Landing Page" }
                        </a>
                    </li>
                    <li>
                        <a href="#" class="text-gray-300 hover:text-white">
                            { "Quem Sou" }
                        </a>
                    </li>
                    <li>
                        <a href="#" class="text-gray-300 hover:text-white">
                            { "Contatos" }
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}

// Componente principal da aplicação
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="bg-gray-100 min-h-screen">
            <Navbar />
            <main class="flex flex-col items-center justify-center min-h-screen bg-gray-100">
                <img class="logo w-32 h-32 mb-8" src="https://yew.rs/img/logo.png" alt="Yew logo" />
                <h1 class="text-3xl font-bold text-gray-800">
                    { "Bem Vindo! Esta Página é Sobre Rust!" }
                </h1>
            </main>
        </div>
    }
}
