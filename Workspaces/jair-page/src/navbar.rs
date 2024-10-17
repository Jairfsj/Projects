use yew::prelude::*;
use yew_router::prelude::*;

//Rotas para navegacao
#[derive(Clone,Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,

}

//Componente da Navbar
#[function_component(Navbar)]
pub fn navbar() -> Html {
    let current-route = use_location().unwrap().route();

    html! {
        <nav>
            <ul class="navbar">
               <li>
                  <Link<Route> to={Route::Home} classes={if current_route == Route::Home { "hidden" } else { "" }}>{ "Home" }</Link<Route>>
               </li>
               <li>
                  <link<Route> to={Route::About} classes={if current_route == Route::About { "hidden" } else { "" }}>{ "Quem Sou Eu" }</Link<Route>>               
               </li>
               <li>
                 <Link<Route> to={Route::Contact} classes=if current_route == Route::Contact { "hidden" } else { "" }}>{ "Contato" }</Link<Route>>
               </li>
            </ul>
        </nav>    
    }
}
