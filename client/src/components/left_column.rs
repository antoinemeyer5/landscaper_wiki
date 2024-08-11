use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::AppRoute;

#[function_component]
pub fn LeftColumn() -> Html {
    html! {
        <div class="column column-left" style="background-color:#ddd;">
            <h1>{ "Landscaper" }</h1>
            <Link<AppRoute> to={AppRoute::PageHome}>
                <div class="column-left-button">{ "🏠" }</div>
            </Link<AppRoute>>
            <Link<AppRoute> to={AppRoute::PagePlants}>
                <div class="column-left-button">{ "🌱" }</div>
            </Link<AppRoute>>
            <Link<AppRoute> to={AppRoute::PageField}>
                <div class="column-left-button">{ "🚜" }</div>
            </Link<AppRoute>>
        </div>
    }
}
