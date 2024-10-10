use yew::prelude::*;

#[function_component(Contatos)]
pub fn contatos() -> Html {
    html! {
        <main class="flex flex-col items-center justify-center flex-grow bg-gradient-to-r from-gray-100 to-gray-300 p-8">
            <h1 class="text-4xl font-bold text-gray-800 mb-6">
                { "Entre em Contato" }
            </h1>
            <p class="text-lg text-gray-600 mb-8 text-center">
                { "Você pode me encontrar nas seguintes plataformas..." }
            </p>
            <ul class="flex space-x-4">
                <li>
                    <a href="https://github.com/Jairfsj" target="_blank" class="btn-accent">
                        { "GitHub" }
                    </a>
                </li>
                <li>
                    <a href="https://linkedin.com/in/Jairfsj" target="_blank" class="btn-accent">
                        { "LinkedIn" }
                    </a>
                </li>
                <li>
                    <a href="mailto:seuemail@example.com" class="btn-accent">
                        { "Email" }
                    </a>
                </li>
            </ul>
        </main>
    }
}
