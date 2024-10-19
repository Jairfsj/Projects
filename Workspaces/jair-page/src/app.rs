use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::{landing_page, about_page, contact_page, navbar, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <Switch<Route> render={Switch::render(switch)} />
        <BrowserRouter>    
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <landing_page.rs },
        Route::About => html! { <about_page /> },
        Route::Contact => html! { <contact_page /> },
    }
}

    
