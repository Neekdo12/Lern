use gloo_net::http::Request;
use shared::Greeting;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let greeting = use_state(|| None::<Greeting>);

    {
        let greeting = greeting.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/api/greeting").send().await {
                    if let Ok(g) = resp.json::<Greeting>().await {
                        greeting.set(Some(g));
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div class="container py-5">
            <h1 class="display-4">
                { greeting.as_ref().map(|g| g.message.clone()).unwrap_or_else(|| "...".to_string()) }
            </h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
