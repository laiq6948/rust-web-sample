use components::header::Header;
use components::todo::todo_list::TodoList;
use yew::{function_component, html};

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <main class="container-fluid mt-2">
                <TodoList />
            </main>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
