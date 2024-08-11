pub mod field;
pub mod home;
pub mod plants;

use yew::prelude::*;
use yew_router::prelude::*;

use field::PageField;
use home::PageHome;
use plants::PagePlants;

use crate::components::{
    header::Header, left_column::LeftColumn,
};

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/field")]
    PageField,
    #[at("/")]
    PageHome,
    #[at("/plants")]
    PagePlants,
    #[not_found]
    #[at("/404")]
    NotFoundPage,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::PageField => html! { <PageField /> },
        AppRoute::PageHome => html! { <PageHome /> },
        AppRoute::PagePlants => html! { <PagePlants /> },
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
