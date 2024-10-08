use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let footer_classes = classes!(
        "sticky",
        "z-50",
        "bottom-0",
        "p-3",
        "bg-white",
        "border-t",
        "border-gray-200",
        "dark:bg-gray-800",
        "dark:border-gray-600",
        "md:flex",
        "md:items-center",
        "md:justify-between",
        "md:p-4"
    );
    html! {
    <footer class={footer_classes}>
      <span class={"text-sm text-gray-500 sm:text-center dark:text-gray-400"}>{"© 2023 "}<a href={"#"} class="hover:underline">{"RT-Jam™"}</a>
      </span>
      <ul class={"flex flex-wrap items-center mt-3 text-sm font-medium text-gray-500 dark:text-gray-400 sm:mt-0"}>
        <li>
          <a href={"#"} class={"hover:underline me-4 md:me-6"}>{"About"}</a>
        </li>
      </ul>
    </footer>
    }
}
