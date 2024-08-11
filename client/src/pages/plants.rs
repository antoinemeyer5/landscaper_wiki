
use yew::prelude::*;

use crate::models::plant::Plant;
use crate::components::plant_list::PlantList;

#[function_component]
pub fn PagePlants() -> Html {
    let plants: Vec<Plant> = vec![
        Plant {
            id: 1,
            name: String::from("Carrot"),
            details: String::from("The carrot is a root vegetable, ..."),
        },
        Plant {
            id: 2,
            name: String::from("Apple"),
            details: String::from("An apple is a round, edible fruit ..."),
        },
        Plant {
            id: 3,
            name: String::from("Rice"),
            details: String::from("Rice is a cereal grain and in its ..."),
        },
    ];

    let selected_plant = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_plant.clone();
        Callback::from(move |plant: Plant| {
            selected_video.set(Some(plant))
        })
    };

    let details = selected_plant.as_ref().map(|plant| html! {
        <p> { format!("{}", plant.details) } </p>
    });

    html! {
        <div>
            <div class="column column-central">
                <h1>{"Plants"}</h1>
                <PlantList plants={plants} on_hover={on_video_select.clone()} />
            </div>
            <div class="column column-right">
                <h1>{"Details"}</h1>
                { for details }
            </div>
        </div>
    }
}
