use yew_router::Routable;


#[derive(Clone, Routable, PartialEq)]
pub enum MainPanelRoute {
    #[at("/")]
    Home,
    #[at("/eventN1")]
    Event1,
}

