use yew::prelude::*;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <>
        <div class="h-lvh w-lvw bg-purple-950 text-white font-source overflow-y-auto">
            <div class="px-4 sm:px-8 md:px-16 lg:px-32 xl:px-64">
                <div class="py-4 md:py-8">
                    <Header />
                        {props.children.clone()}
                    <Footer />
                </div>
            </div>
        </div>
        </>
    }
}

const LATEST_LINK: &str = "https://www.eventbrite.co/e/nostr-dev-reunion-el-salvador-aprender-y-conocer-tickets-884565548367?aff=oddtdtcreator";

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="w-full flex flex-row justify-between py-4 my-4 sm:py-8 sm:my-8 border-stone-100 border-b-2  border-dashed">
            <h1 class="text-xl sm:text-2xl md:text-3xl lg:text-4xl font-bold">{"NostrDevs El Salvador"}</h1>
            <a
                target="_blank"
                href={LATEST_LINK}
                class="align-baseline text-purple-300 hover:text-purple-600">{"Meetup"}</a>
        </div>
    }
}

const GITHUB_LINK: &str = "https://github.com/42Pupusas/NostrDevs";
const X_LINK: &str = "https://twitter.com/illuminodes";

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
            <div class="w-full flex flex-row gap-8  py-4 my-8 sm:py-8 sm:my-16 border-stone-100 border-t-2  border-dashed">
                // GITHUB LOGO
                <a href={GITHUB_LINK} target="_blank">
                <svg class="svgIcon" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                    <path fill-rule="evenodd" d="M12.006 2a9.847 9.847 0 0 0-6.484 2.44 10.32 10.32 0 0 0-3.393 6.17 10.48 10.48 0 0 0 1.317 6.955 10.045 10.045 0 0 0 5.4 4.418c.504.095.683-.223.683-.494 0-.245-.01-1.052-.014-1.908-2.78.62-3.366-1.21-3.366-1.21a2.711 2.711 0 0 0-1.11-1.5c-.907-.637.07-.621.07-.621.317.044.62.163.885.346.266.183.487.426.647.71.135.253.318.476.538.655a2.079 2.079 0 0 0 2.37.196c.045-.52.27-1.006.635-1.37-2.219-.259-4.554-1.138-4.554-5.07a4.022 4.022 0 0 1 1.031-2.75 3.77 3.77 0 0 1 .096-2.713s.839-.275 2.749 1.05a9.26 9.26 0 0 1 5.004 0c1.906-1.325 2.74-1.05 2.74-1.05.37.858.406 1.828.101 2.713a4.017 4.017 0 0 1 1.029 2.75c0 3.939-2.339 4.805-4.564 5.058a2.471 2.471 0 0 1 .679 1.897c0 1.372-.012 2.477-.012 2.814 0 .272.18.592.687.492a10.05 10.05 0 0 0 5.388-4.421 10.473 10.473 0 0 0 1.313-6.948 10.32 10.32 0 0 0-3.39-6.165A9.847 9.847 0 0 0 12.007 2Z" clip-rule="evenodd"/>
                </svg>
                </a>

                // X LOGO
                <a href={X_LINK} target="_blank">
                <svg class="svgIcon" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M13.795 10.533 20.68 2h-3.073l-5.255 6.517L7.69 2H1l7.806 10.91L1.47 22h3.074l5.705-7.07L15.31 22H22l-8.205-11.467Zm-2.38 2.95L9.97 11.464 4.36 3.627h2.31l4.528 6.317 1.443 2.02 6.018 8.409h-2.31l-4.934-6.89Z"/>
                </svg>
                </a>
                // NOSTR LOGO

            </div>
        }
}
