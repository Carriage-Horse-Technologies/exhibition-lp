use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let HeaderProps {} = props;
    html! {
        <header class="fixed w-screen mx-auto p-5 text-6xl text-center text-dark-primary font-bold
            text-transparent bg-clip-text bg-gradient-to-r from-green-600 via-yellow-400 to-pink-600
            dark:bg-opacity-0 z-[950]">
            {"ハッカソン成果物展示場"}
        </header>
    }
}
