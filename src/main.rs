use yew::prelude::*;

mod components;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <components::Message
                person={ "John Fakeland" }
                message={ "Ah yes, this is the perfect place to build." }
            />
            <components::Message
                person={ "Jane Fakeland" }
                message={ "Ok, and?" }
            />
            <components::Message
                person={ "John Fakeland" }
                message={ "We could create a country that is different from all of the others!" }
            />
            <components::Message
                person={ "Jane Fakeland" }
                message={ "Do you know that every founder of every country said that, and then it winds up being literally the same as all of the others." }
            />
            <components::Message
                person={ "John Fakeland" }
                message={ "It's not going to be the same like the others, we can have a proper economy, a good following, all while just simply being different!" }
            />
            <components::Message
                person={ "Jane Fakeland" }
                message={ "Okay, let's try it." }
            />

            <h1>{ "Start!" }</h1>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
