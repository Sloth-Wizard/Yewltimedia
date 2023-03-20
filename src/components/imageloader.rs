use std::fmt::Error;
use yew::prelude::*;
use yew::{Component, Context};

use crate::models::imageloader::*;

pub type Binary = Result<Vec<u8>, Error>;
pub enum LazyImage {
    Load(String)
}

pub struct ImageLoader {
    image: ImageProps
}

impl Component for ImageLoader {
    type Message = LazyImage;
    type Properties = ImageProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            image: ctx.props().to_owned()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LazyImage::Load(src) => {
                // Get request to retreive image as a binary and send it to the image component
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let image = self.image.clone();

        html! {
            <img src="" alt={image.path} />
        }
    }
}
