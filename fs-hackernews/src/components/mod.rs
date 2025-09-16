//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod hero;
pub use hero::Hero;

mod echo;
pub use echo::Echo;

mod home;
pub use home::HomePage;

mod loading;
pub use loading::LoadingIndicator;

mod child_or_load;
pub use child_or_load::ChildrenOrLoading;

mod comment;
pub use comment::Comment;

mod preview;
pub use preview::Preview;

mod listing;
pub use listing::StoryListing;
