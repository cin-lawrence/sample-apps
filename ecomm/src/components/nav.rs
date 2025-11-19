use dioxus::prelude::*;

use crate::components::{
    IconCart,
    Icon5,
    Icon6,
    Icon7,
    Icon8,
    Icon9,
    Icon10,
    Icon11,
    Icon12,
    Icon13,
};


#[allow(non_snake_case)]
pub fn Nav() -> Element {
    rsx! {
        section { class: "relative",
            nav { class: "flex justify-between border-b",
                div { class: "px-12 py-8 flex w-full items-center",
                    a { class: "hidden xl:block mr-16",
                        href: "/",
                        IconCart {}
                    }
                    ul { class: "hidden xl:flex font-semibold font-heading",
                        li { class: "mr-12",
                            a { class: "hover:text-gray-600",
                                href: "/",
                                "Category"
                            }
                        }
                        li { class: "mr-12",
                            a { class: "hover:text-gray-600",
                                href: "/",
                                "Collection"
                            }
                        }
                        li { class: "mr-12",
                            a { class: "hover:text-gray-600",
                                href: "/",
                                "Story"
                            }
                        }
                        li {
                            a { class: "hover:text-gray-600",
                                href: "/",
                                "Brand"
                            }
                        }
                    }
                    a { class: "flex-shrink-0 xl:mx-auto text-3xl font-bold font-heading",
                        href: "/",
                        img { class: "h-9",
                            width: "auto",
                            alt: "",
                            src: "https://shuffle.dev/yofte-assets/logos/yofte-logo.svg",
                        }
                    }
                    div { class: "hidden xl:inline-block mr-14",
                        input { class: "py-5 px-8 w-full placeholder-gray-400 text-xs uppercase font-semibold font-heading bg-gray-50 border border-gray-200 focus:ring-blue-300 focus:border-blue-300 rounded-md",
                            placeholder: "Search",
                            r#type: "text",
                        }
                    }
                    div { class: "hidden xl:flex items-center",
                        a { class: "mr-10 hover:text-gray-600",
                            href: "",
                            Icon5 {}
                        }
                        a { class: "flex items-center hover:text-gray-600",
                            href: "/",
                            Icon6 {}
                            span { class: "inline-block w-6 h-6 text-center bg-gray-50 rounded-full font-semibold font-heading",
                                "3"
                            }
                        }
                    }
                }
                a { class: "hidden xl:flex items-center px-12 border-l font-semibold font-heading hover:text-gray-600",
                    href: "/",
                    Icon7 {}
                    span {
                        "Sign In"
                    }
                }
                a { class: "xl:hidden flex mr-6 items-center text-gray-600",
                    href: "/",
                    Icon8 {}
                    span { class: "inline-block w-6 h-6 text-center bg-gray-50 rounded-full font-semibold font-heading",
                        "3"
                    }
                }
                a { class: "navbar-burger self-center mr-12 xl:hidden",
                    href: "/",
                    Icon9 {}
                }
            }
            div { class: "hidden navbar-menu fixed top-0 left-0 bottom-0 w-5/6 max-w-sm z-50",
                div { class: "navbar-backdrop fixed inset-0 bg-gray-800 opacity-25",
                }
                nav { class: "relative flex flex-col py-6 px-6 w-full h-full bg-white border-r overflow-y-auto",
                    div { class: "flex items-center mb-8",
                        a { class: "mr-auto text-3xl font-bold font-heading",
                            href: "/",
                            img { class: "h-9",
                                src: "https://shuffle.dev/yofte-assets/logos/yofte-logo.svg",
                                width: "auto",
                                alt: "",
                            }
                        }
                        button { class: "navbar-close",
                            Icon10 {}
                        }
                    }
                    div { class: "flex mb-8 justify-between",
                        a { class: "inline-flex items-center font-semibold font-heading",
                            href: "/",
                            Icon11 {}
                            span {
                                "Sign In"
                            }
                        }
                        div { class: "flex items-center",
                            a { class: "mr-10",
                                href: "/",
                                Icon12 {}
                            }
                            a { class: "flex items-center",
                                href: "/",
                                Icon13 {}
                                span { class: "inline-block w-6 h-6 text-center bg-gray-100 rounded-full font-semibold font-heading",
                                    "3"
                                }
                            }
                        }
                    }
                    input { class: "block mb-10 py-5 px-8 bg-gray-100 rounded-md border-transparent focus:ring-blue-300 focus:border-blue-300 focus:outline-none",
                        r#type: "search",
                        placeholder: "Search",
                    }
                    ul { class: "text-3xl font-bold font-heading",
                        li { class: "mb-8",
                            a {
                                href: "/",
                                "Category"
                            }
                        }
                        li { class: "mb-8",
                            a {
                                href: "/",
                                "Collection"
                            }
                        }
                        li { class: "mb-8",
                            a {
                                href: "/",
                                "Story"
                            }
                        }
                        li {
                            a {
                                href: "/",
                                "Brand"
                            }
                        }
                    }
                }
            }
        }
    }
}
