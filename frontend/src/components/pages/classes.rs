use yew::{classes, Classes};

pub fn main_div_classes() -> Classes {
    classes!(
        "flex",
        "flex-col",
        "h-screen",
        "flex-grow",
        "items-center",
        "justify-center",
        "px-6",
        "py-8",
        "mx-auto",
        "lg:py-0"
    )
}

pub fn box_div_classes() -> Classes {
    classes!(
        "w-full",
        "bg-white",
        "rounded-lg",
        "shadow",
        "dark:border",
        "md:mt-0",
        "sm:max-w-md",
        "xl:p-0",
        "dark:bg-gray-800",
        "dark:border-gray-700"
    )
}

pub fn submit_button_classes() -> Classes {
    classes!(
        "w-full",
        "text-white",
        "bg-primary-600",
        "hover:bg-primary-700",
        "focus:ring-4",
        "focus:outline-none",
        "focus:ring-primary-300",
        "font-medium",
        "rounded-lg",
        "text-sm",
        "px-5",
        "py-2.5",
        "text-center",
        "dark:bg-primary-600",
        "dark:hover:bg-primary-700",
        "dark:focus:ring-primary-800"
    )
}
