use yew::{classes, Classes};

pub fn text_input_classes() -> Classes {
    classes!(
        "bg-gray-50",
        "border",
        "border-gray-300",
        "text-gray-900",
        "sm:text-sm",
        "rounded-lg",
        "focus:ring-primary-600",
        "focus:border-primary-600",
        "block",
        "w-full",
        "p-2.5",
        "dark:bg-gray-700",
        "dark:border-gray-600",
        "dark:placeholder-gray-400",
        "dark:text-white",
        "dark:focus:ring-blue-500",
        "dark:focus:border-blue-500"
    )
}

pub fn input_error_classes() -> Classes {
    classes!("text-red-500", "text-xs", "pt-1", "block")
}

pub fn label_classes() -> Classes {
    classes!(
        "block",
        "mb-2",
        "text-sm",
        "font-medium",
        "text-gray-900",
        "dark:text-white"
    )
}
