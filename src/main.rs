mod components;
mod views;

fn main() {
    yew::Renderer::<views::frontpage::FrontPage>::new().render();
}
