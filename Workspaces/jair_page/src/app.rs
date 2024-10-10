use yew::prelude::*;
use yew_router::prelude::*;

// Definição das rotas
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    LandingPage,
    #[at("/about")]
    QuemSou,
    #[at("/contact")]
    Contatos,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// Função de renderização das rotas
fn switch(routes: Route) -> Html {
    match routes {
        Route::LandingPage => html! { <landing_page::LandingPage /> },
        Route::QuemSou => html! { <quem_sou::QuemSou /> },
        Route::Contatos => html! { <contatos::Contatos /> },
        Route::NotFound => html! { <h1>{ "404 - Página não encontrada" }</h1> },
    }
}

// Componente principal da aplicação
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="min-h-screen flex flex-col">
                <Navbar />
                <Switch<Route> render={Switch::render(switch)} />
            </div>
        </BrowserRouter>
    }
}

// Componente Navbar
#[function_component(Navbar)]
fn navbar() -> Html {
    let current_route = use_route::<Route>();

    html! {
        <nav class="bg-gray-800 p-4">
            <div class="container mx-auto flex justify-between items-center">
                <div class="text-white text-lg font-semibold">
                    { "Meu Portfólio" }
                </div>
                <ul class="flex space-x-4">
                    <li>
                        {
                            if current_route.as_ref() != Some(&Route::LandingPage) {
                                html! {
                                    <Link<Route> to={Route::LandingPage} classes="btn-primary">
                                        { "Landing Page" }
                                    </Link<Route>>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </li>
                    <li>
                        {
                            if current_route.as_ref() != Some(&Route::QuemSou) {
                                html! {
                                    <Link<Route> to={Route::QuemSou} classes="btn-primary">
                                        { "Quem Sou" }
                                    </Link<Route>>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </li>
                    <li>
                        {
                            if current_route.as_ref() != Some(&Route::Contatos) {
                                html! {
                                    <Link<Route> to={Route::Contatos} classes="btn-primary">
                                        { "Contatos" }
                                    </Link<Route>>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </li>
                </ul>
            </div>
        </nav>
    }
}
