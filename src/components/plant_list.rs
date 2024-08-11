use yew::prelude::*;

use crate::models::plant::Plant;

#[derive(Properties, PartialEq)]
pub struct PlantsListProps {
    pub plants: Vec<Plant>,
    pub on_hover: Callback<Plant>
}

#[function_component]
pub fn PlantList(PlantsListProps { plants, on_hover }: &PlantsListProps) -> Html {

    plants.iter().map(|plant: &Plant| {


        let on_select = {
            let on_hover = on_hover.clone();
            let plant: Plant = plant.clone();
            Callback::from(move |_| { on_hover.emit(plant.clone()) })
        };

        html! {
            <div class="plant-item" onmouseover={on_select}>
                <p key={plant.id} style="margin-right: 160px;">
                    {format!("🌱 {}", plant.name)}
                </p>
            </div>
        }
    }).collect()
}
