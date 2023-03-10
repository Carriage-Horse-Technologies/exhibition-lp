use yew_router::Routable;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/entrance")]
    Entrance,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}
