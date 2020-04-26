use web_sys::HtmlImageElement;
use wasm_bindgen::JsCast;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use futures::channel::oneshot::Sender;
use wasm_bindgen::JsValue;

/// This struct represent an image.
/// It is useful when using the [Sprite struct](../sprite/struct.Sprite.html).
/// 
/// # Example
/// 
/// ```rust
/// use wasm_game_lib::graphics::image::Image;
/// 
/// # async fn test() {
/// // load a texture encoded in text
/// let ferris = Image::load("data:image/png;base64,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").await.unwrap();
/// // load a texture from the web
/// let ferris2 = Image::load("https://rustacean.net/assets/cuddlyferris.svg").await.unwrap();
/// # }
/// ```
#[derive(Debug, PartialEq)]
pub struct Image {
    element: HtmlImageElement,
}

impl Image {
    /// Load an Image.
    /// Return a Result because it may fail.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// # use wasm_game_lib::graphics::image::Image;
    /// # async fn test() {
    /// // load a texture encoded in text
    /// let ferris = Image::load("data:image/png;base64,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").await.unwrap();
    /// // load a texture from the web
    /// let ferris2 = Image::load("https://rustacean.net/assets/cuddlyferris.svg").await.unwrap();
    /// # }
    /// ```
    pub async fn load(url: &str) -> Result<Image, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document
            .create_element("img")
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        element.set_src(url);

        JsFuture::from(Promise::new(&mut |yes, no| {
            element.add_event_listener_with_callback("load", &yes).unwrap();
            element.add_event_listener_with_callback("error", &no).unwrap();
        })).await;

        Ok(Image {
            element
        })
    }

    /// Load an Image and send it trought a [oneshot channel](https://docs.rs/futures/0.3.4/futures/channel/oneshot/fn.channel.html).
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use wasm_game_lib::graphics::image::Image;
    /// use wasm_game_lib::system::sleep;
    /// use futures::channel::oneshot::Receiver;
    /// use futures::channel::oneshot;
    /// use futures::join;
    /// use wasm_bindgen::JsValue;
    /// use std::time::Duration;
    /// 
    /// // the function which will be executed during the load
    /// async fn loading_tracker(mut receivers: Vec<Receiver<Result<Image, JsValue>>>) -> Vec<Result<Image, JsValue>> {
    ///     let mut images = Vec::new();
    ///     for _ in 0..receivers.len() {
    ///         images.push(None);
    ///     }
    /// 
    ///     loop {
    ///         for i in 0..images.len() {
    ///             if images[i].is_none() {
    ///                 if let Ok(Some(result)) = receivers[i].try_recv() {
    ///                     images[i] = Some(result);
    ///                 }
    ///             }
    ///         }
    ///         
    ///         if !images.contains(&None) {
    ///             // break when every image is ready
    ///             break;
    ///         }
    /// 
    ///         // you may want to display a progress bar here
    /// 
    ///         sleep(Duration::from_millis(20)).await;
    ///     }
    /// 
    ///     let mut unwraped_images = Vec::new();
    ///     for image in images {
    ///         unwraped_images.push(image.unwrap());
    ///     }
    /// 
    ///     return unwraped_images;
    /// }
    /// 
    /// async fn start() {
    ///     // create 2 oneshot channels
    ///     let (sender1, receiver1) = oneshot::channel::<Result<Image, JsValue>>();
    ///     let (sender2, receiver2) = oneshot::channel::<Result<Image, JsValue>>();
    ///     
    ///     // create futures
    ///     let loading_tracker_future = loading_tracker(vec![receiver1, receiver2]);
    ///     let image1_future = Image::load_and_send("https://images.pexels.com/photos/1086723/pexels-photo-1086723.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=7000&w=7000", sender1);
    ///     let image2_future = Image::load_and_send("https://c.wallhere.com/photos/c1/ff/Moon_rocks_sky_8k-1430191.jpg!d", sender2);
    ///     
    ///     // execute the three futures simultaneously and get the images
    ///     let images = join!(loading_tracker_future, image1_future, image2_future).0;
    /// }
    /// ```
    pub async fn load_and_send(url: &str, sender: Sender<Result<Image, JsValue>>) {
        let image = Image::load(url).await;
        sender.send(image).expect("can't send the loaded image trought the oneshot shannel");
    }

    pub fn get_html_element(&self) -> &HtmlImageElement {
        &self.element
    }

    /// Return the width of the image.
    pub fn get_width<T: From<u32>>(&self) -> T {
        self.element.width().into()
    }

    /// Return the height of the image.
    pub fn get_height<T: From<u32>>(&self) -> T {
        self.element.height().into()
    }

    /// Return a tuple containing width and height.
    pub fn get_size<T: From<u32>>(&self) -> (T, T) {
        (self.element.width().into(), self.element.height().into())
    }
}