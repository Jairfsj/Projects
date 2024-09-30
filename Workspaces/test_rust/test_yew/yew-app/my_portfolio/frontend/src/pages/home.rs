use yew::prelude::*;
use crate::components::{Header, Footer, Chat};

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self:Properties, _: ComponentLink<Self>) -> Self {
        Home
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender{
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header />
                <main>
                    <h1>{"Bem-vindo ao meu portfolio"}</h1>
                    <p>{"Aqui estao algumas habilidas que possuo"}</p> 
                    <ul>
                        
                        <li>{"Rust"}</li>
                        <li>{"WebAssembly"}</li>
                        <li>{"Actix Web"}</li>
                        <li>{"Yew"}</li>
                    </ul> 
                    <Chat />
                </main>
            </>    <Footer />
        
        }
    }

}
