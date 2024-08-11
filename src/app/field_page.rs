use yew::prelude::*;

#[function_component]
pub fn FieldPage() -> Html {
    let field: Vec<Vec<String>> = vec![
        vec![
            String::from("a"),
            String::from("b"),
        ],
        vec![
            String::from("c"),
            String::from("d"),
        ]
    ];

    html! {
        <div class="column column-central">
            <h1>{"Field"}</h1>
            <p>{"todo field"}</p>
        </div>
    }
}
