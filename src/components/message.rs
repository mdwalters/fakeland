use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub person: String,
    pub message: String
}

#[function_component]
pub fn Message(props: &Props) -> Html {
    html! {
        <>
            <p class="faded">{ props.person.clone() }</p>
            <p>{ props.message.clone() }</p>
            <br />
        </>
    }
}
