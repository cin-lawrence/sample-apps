use dioxus::prelude::*;
use crate::components::{Nav, ProductPage};

#[derive(PartialEq, Clone, Props)]
pub struct DetailsProps {
    product_id: usize,
}

#[allow(non_snake_case)]
pub fn Details(props: DetailsProps) -> Element {
    let product_id = props.product_id;

    rsx! {
        div {
            Nav {}
            ProductPage {
                product_id,
            }
        }
    }
}
