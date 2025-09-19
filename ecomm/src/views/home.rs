use dioxus::prelude::*;

use crate::enums::Sort;
use crate::components::Nav;
use crate::components::ProductItem;
use crate::models::Product;
use crate::services::FakeStoreAPI;


#[allow(non_snake_case)]
pub fn Home() -> Element {
    let products: Resource<dioxus::Result<Vec<Product>>> = use_server_future(
        move || async move {
            Ok(FakeStoreAPI::fetch_products(10, Sort::Ascending).await?)
        }
    )?;
    let products = products().unwrap()?;

    rsx! {
        Nav {}
        section { class: "p-10",
            for product in products {
                ProductItem {
                    product
                }
            }
        }
    }
}
