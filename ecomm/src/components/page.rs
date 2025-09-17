use dioxus::prelude::*;
use crate::services::FakeStoreAPI;
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
    }
}
