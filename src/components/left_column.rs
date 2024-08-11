use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

#[function_component]
pub fn LeftColumn() -> Html {
    html! {
        <div class="column column-left" style="background-color:#ddd;">
            <h1>{ "Landscaper" }</h1>
            <Link<AppRoute> to={AppRoute::HomePage}>
                <div class="column-left-button">{ "🏠" }</div>
            </Link<AppRoute>>
            <Link<AppRoute> to={AppRoute::PlantPage}>
                <div class="column-left-button">{ "🌱" }</div>
            </Link<AppRoute>>
            <Link<AppRoute> to={AppRoute::FieldPage}>
                <div class="column-left-button">{ "🚜" }</div>
            </Link<AppRoute>>
        </div>
    }
}
