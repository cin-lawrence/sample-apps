use dioxus::prelude::*;
use crate::services::FakeStoreAPI;
use crate::components::{Icon1, Icon2, Icon3, Icon4};
use crate::models::Product;
use crate::enums::Size;

#[derive(PartialEq, Clone, Props)]
pub struct ProductPageProps {
    product_id: ReadOnlySignal<usize>,
}

#[allow(non_snake_case)]
pub fn ProductPage(props: ProductPageProps) -> Element {
    let mut quantity = use_signal(|| 1);
    let mut size = use_signal(Size::default);

    let product: Resource<dioxus::Result<Product>> = use_server_future(
        move || async move {
            Ok(FakeStoreAPI::fetch_product((props.product_id)()).await?)
        }
    )?;

    let Product {
        title,
        price,
        description,
        category,
        image,
        rating,
        ..
    } = product().unwrap()?;

    rsx! {
        section { class: "py-20",
            div { class: "container mx-auto px-4",
                div { class: "flex flex-wrap -mx-4 mb-24",
                    div { class: "w-full md:w-1/2 px-4 mb-8 md:mb-0",
                        div { class: "relative mb-10",
                            style: "height: 564px;",
                            a { class: "absolute top-1/2 left-0 ml-8 transform translate-1/2",
                                href: "#",
                                Icon1 {}
                            }
                            img { class: "object-cover w-full h-full",
                                alt: "",
                                src: "{image}",
                            }
                            a { class: "absolute top-1/2 right-0 mr-8 transform translate-1/2",
                                href: "#",
                                Icon2 {}
                            }
                        }
                    }
                    div { class: "w-full md:w-1/2 px-4",
                        div { class: "lg:pl-20",
                            div { class: "mb-10 pb-10 border-b",
                                h2 { class: "mt-2 mb-6 max-w-xl text-5xl md:text-6xl font-bold font-heading",
                                    "{title}"
                                }
                                div { class: "mb-8",
                                    "{rating}"
                                }
                                p { class: "inline-block mb-8 text-2xl font-bold font-heading text-blue-300",
                                    span {
                                        "${price}"
                                    }
                                }
                                p { class: "max-w-md text-gray-500",
                                    "{description}"
                                }
                            }
                            div { class: "flex mb-12",
                                div { class: "mr-6",
                                    span { class: "block mb-4 font-bold font-heading text-gray-400 uppercase",
                                        "QTY"
                                    }
                                    div { class: "inline-flex items-center px-4 font-semibold font-heading text-gray-500 border border-gray-200 focus:ring-blue-300 focus:border-blue-300 rounded-md",
                                        button { class: "py-2 hover:text-gray-700",
                                            onclick: move |_| quantity += 1,
                                            Icon3 {}
                                        }
                                        input { class: "w-12 m-0 px-2 py-4 text-center md:text-right border-0 focus:ring-transparent focus:outline-none rounded-md",
                                            placeholder: "1",
                                            r#type: "number",
                                            value: "{quantity}",
                                            oninput: move |evt| if let Ok(as_number) = evt.value().parse() { quantity.set(as_number) },
                                        }
                                        button { class: "py-2 hover:text-gray-700",
                                            onclick: move |_| quantity -= 1,
                                            Icon4 {}
                                        }
                                    }
                                }
                                div {
                                    span { class: "block mb-4 font-bold font-heading text-gray-400 uppercase",
                                        "Size"
                                    }
                                    select { class: "pl-6 pr-10 py-4 font-semibold font-heading text-gray-500 border border-gray-200 focus:ring-blue-300 focus:border-blue-300 rounded-md",
                                        id: "",
                                        name: "",
                                        onchange: move |evt| {
                                            if let Ok(new_size) = evt.value().parse() {
                                                size.set(new_size);
                                            }
                                        },
                                        option {
                                            value: "1",
                                            "Medium"
                                        }
                                        option {
                                            value: "2",
                                            "Small"
                                        }
                                        option {
                                            value: "3",
                                            "Large"
                                        }
                                    }
                                }
                            }
                            div { class: "flex flex-wrap -mx-4 mb-14 items-center",
                                div { class: "w-full xl:w-2/3 px-4 mb-4 xl:mb-0",
                                    a { class: "block bg-orange-300 hover:bg-orange-400 text-center text-white font-bold font-heading py-5 px-8 rounded-md uppercase transition duration-200",
                                        href: "#",
                                        "Add to cart"
                                    }
                                }
                            }
                            div { class: "flex items-center",
                                span { class: "mr-8 text-gray-500 font-bold font-heading uppercase",
                                    "SHARE IT"
                                }
                                a { class: "mr-1 w-8 h-8",
                                    href: "#",
                                    img {
                                        alt: "",
                                        src: "https://shuffle.dev/yofte-assets/buttons/facebook-circle.svg",
                                    }
                                }
                                a { class: "mr-1 w-8 h-8",
                                    href: "#",
                                    img {
                                        alt: "",
                                        src: "https://shuffle.dev/yofte-assets/buttons/instagram-circle.svg",
                                    }
                                }
                                a { class: "w-8 h-8",
                                    href: "#",
                                    img {
                                        src: "https://shuffle.dev/yofte-assets/buttons/twitter-circle.svg",
                                        alt: "",
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    ul { class: "flex flex-wrap mb-16 border-b-2",
                        li { class: "w-1/2 md:w-auto",
                            a { class: "inline-block py-6 px-10 bg-white text-gray-500 font-bold font-heading shadow-2xl",
                                href: "#",
                                "Description"
                            }
                        }
                        li { class: "w-1/2 md:w-auto",
                            a { class: "inline-block py-6 px-10 text-gray-500 font-bold font-heading",
                                href: "#",
                                "Customer reviews"
                            }
                        }
                        li { class: "w-1/2 md:w-auto",
                            a { class: "inline-block py-6 px-10 text-gray-500 font-bold font-heading",
                                href: "#",
                                "Shipping & returns"
                            }
                        }
                        li { class: "w-1/2 md:w-auto",
                            a { class: "inline-block py-6 px-10 text-gray-500 font-bold font-heading",
                                href: "#",
                                "Brand"
                            }
                        }
                    }
                    h3 { class: "mb-8 text-3xl font-bold font-heading text-blue-300",
                        "{category}"
                    }
                    p { class: "max-w-2xl text-gray-500",
                        "{description}"
                    }
                }
            }
        }
    }
}
