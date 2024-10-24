use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "A disposicao !" }</h1>
            <span class="subtitle">{ "Bem Vindo! " }<i class="heart" /></span>
            <body>
                <header id="navbar">
                <nav class="navbar-container container">
                    <a href="/" class="home-link">
                    <div class="navbar-logo"></div>
                    //Website Name
                    </a>
                    <button
                    type="button"
                    id="navbar-toggle"
                    aria-controls="navbar-menu"
                    aria-label="Toggle menu"
                    aria-expanded="false"
                    >
                    <span class="icon-bar"></span>
                    <span class="icon-bar"></span>
                    <span class="icon-bar"></span>
                    </button>
                    <div id="navbar-menu" aria-labelledby="navbar-toggle">
                    <ul class="navbar-links">
                        <li class="navbar-item"><a class="navbar-link" href="/" ></a></li>
                        <li class="navbar-item"><a class="navbar-link" href="/" ></a></li>
                        <li class="navbar-item"><a class="navbar-link" href="/" ></a></li>
                        <li class="navbar-item"><a class="navbar-link" href="/" ></a></li>
                    </ul>
                    </div>
                </nav>
                </header>
                <script src="index.js"></script>
            </body>

        </main>

    }
}
