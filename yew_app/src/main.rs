use yew::prelude::*;

struct CounterComponent {
    count: i64,
}

impl Component for CounterComponent {
    type Message    = Msg;
    type Properties = (); //data passed in from
                            //parent component
    //life cycle method
    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                   self.count += 1;
                   true // re-render component
            }
        }
    }
    // similar to render function in React, it defines
    // how a component should be rendered
    fn view(&self, ctx: &Context<Self>) -> Html {
        //when the button is clicked we want to 
        //increment the counter and re-render the component
        // a reference to scope is needed here
        let link = ctx.link(); //type: &Scope<CounterComponent>
        html! {
            <div clas="container">
              <p> { self.count } </p>
              <button onclick={link.callback(|_| Msg::AddOne)}> { "+1"} </button>
            </div>
        }
    }
}
// msgs can be dispatched based
// on UI envents
enum Msg {
    AddOne
}

fn main() {
    //I need to pass the route component here
    yew::start_app::<CounterComponent>();
}
