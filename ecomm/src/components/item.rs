use dioxus::prelude::*;

use crate::models::Product;

#[derive(PartialEq, Clone, Props)]
pub struct ProductItemProps {
    product: Product,
}

#[allow(non_snake_case)]
pub fn ProductItem(props: ProductItemProps) -> Element {
    let Product {
        id,
        title,
        price,
        category,
        image,
        rating,
        ..
    } = props.product;

    rsx! {
        section { class: "h-40 p-2 m-2 shadow-lg ring-1 rounded-lg flex flex-row place-items-center hover:ring-4 hover:shadow-2xl transition-all duration-200",
            img {
                class: "object-scale-down w-1/6 h-full",
                src: "{image}",
            }
            div { class: "pl-4 text-left text-ellipsis",
                a {
                    href: "/details/{id}",
                    class: "w-full text-center",
                    "{title}"
                }
                p {
                    class: "w-full",
                    "{rating}"
                }
                p {
                    class: "w-full",
                    "{category}"
                }
                p {
                    class: "w-1/4",
                    "${price}"
                }
            }
        }
    }
}
