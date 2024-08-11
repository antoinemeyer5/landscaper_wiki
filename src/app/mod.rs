pub mod field_page;
pub mod home_page;
pub mod plant_page;

use yew::prelude::*;
use yew_router::prelude::*;

use field_page::FieldPage;
use home_page::HomePage;
use plant_page::PlantPage;

use crate::components::{
    header::Header, left_column::LeftColumn,
};

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/field")]
    FieldPage,
    #[at("/")]
    HomePage,
    #[at("/plants")]
    PlantPage,
    #[not_found]
    #[at("/404")]
    NotFoundPage,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::FieldPage => html! { <FieldPage /> },
        AppRoute::HomePage => html! { <HomePage /> },
        AppRoute::PlantPage => html! { <PlantPage /> },
        AppRoute::NotFoundPage => html! {
            <div class="column column-central"> {"Page not found"} </div>
        },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <HashRouter>
            <Header />
            <div class="row">
                <LeftColumn />
                <Switch<AppRoute> render={switch} />
            </div>
        </HashRouter>
    }
}
