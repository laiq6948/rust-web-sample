use std::fmt::Debug;
use yew::{html, Component, Context, Html, InputEvent};

#[derive(Clone, Debug, Default)]
pub struct Todo {
    pub title: String,
    pub msg: String,
}

impl Todo {
    pub fn render(&self) -> Html {
        html! {
            <div>
                <p>{ format!("title: {}, msg : {}", self.title ,self.msg) }</p>
            </div>
        }
    }
}

struct Model {
    todos: Vec<Todo>,
    title: String,
    msg: String,
}

#[derive(Debug)]
enum Msg {
    PostTodo,
    SetTitle(String),
    SetMsg(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let todos: Vec<Todo> = Vec::new();
        Self {
            todos: todos,
            title: String::new(),
            msg: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // log::info!("Update: {:?}", msg);
        match msg {
            Msg::PostTodo => {
                let todo = Todo {
                    title: self.title.clone(),
                    msg: self.msg.clone(),
                };
                self.todos.push(todo);
                self.title.clear();
                self.msg.clear();
            }
            Msg::SetTitle(value) => {
                // log::info!("value: {:?}", value);
                self.title = value;
            }
            Msg::SetMsg(value) => {
                // log::info!("value: {:?}", value);
                self.msg = value;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Todo App"}</h1>
                    {"title :"}  <input type="text" value={self.title.clone()} oninput={ctx.link().callback(|e: InputEvent| Msg::SetTitle(e.data().unwrap_or("".to_string())))}/><br/>
                    {"msg :"}  <input type="text" value={self.msg.clone()} oninput={ctx.link().callback(|e: InputEvent| Msg::SetMsg(e.data().unwrap_or("".to_string())))}/><br/>
                    <button onclick={ctx.link().callback(|_| Msg::PostTodo)}>{ "Post Todos" }</button>
                <div>
                    { for self.todos.iter().map(Todo::render) }
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
