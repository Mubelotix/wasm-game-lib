use wasm_bindgen::JsValue;

/// This struct represent a font.
/// It is useful when using the [Text struct](../text/struct.Text.html).
/// 
/// # Example
/// 
/// ```rust
/// use wasm_game_lib::graphics::font::Font;
/// use wasm_game_lib::graphics::text::Text;
/// 
/// # async fn test() {
/// // use a default font
/// let arial = Font::arial();
/// 
/// // load a custom font
/// let trade_winds = Font::load("https://fonts.gstatic.com/s/tradewinds/v8/AYCPpXPpYNIIT7h8-QenM0Jt5vM.woff2").await.unwrap();
/// 
/// // use the fonts with the Text struct
/// let arial_text = Text::new_with_text_and_coords(&arial, "This text is using Arial font.", (0,100));
/// let trade_winds_text = Text::new_with_text_and_coords(&trade_winds, "This text is using Trade Winds font.", (0,200));
/// # }
/// ```
pub struct Font {
    pub(crate) name: String
}

impl Font {
    /// Load a custom font from an url.
    /// The load may fail so a Result is returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use wasm_game_lib::graphics::font::Font;
    /// use wasm_game_lib::graphics::text::Text;
    /// 
    /// # async fn test() {
    /// // load the font
    /// let trade_winds = Font::load("https://fonts.gstatic.com/s/tradewinds/v8/AYCPpXPpYNIIT7h8-QenM0Jt5vM.woff2").await.unwrap();
    /// 
    /// // use the font with the Text struct
    /// let trade_winds_text = Text::new_with_text_and_coords(&trade_winds, "This text is using Trade Winds font.", (0,100));
    /// # }
    /// ```
    /// 
    /// # About Google Fonts
    /// 
    /// If you are using google fonts, you may have a link like "https://fonts.googleapis.com/css?family=Trade+Winds&display=swap".
    /// To get the font url from this link, load this link in a web browser. 
    /// You should get something like that:
    /// ```text
    /// /* latin */
    /// @font-face {
    ///     font-family: 'Trade Winds';
    ///     font-style: normal;
    ///     font-weight: 400;
    ///     font-display: swap;
    ///     src: local('Trade Winds'), local('TradeWinds'), url(https://fonts.gstatic.com/s/tradewinds/v8/AYCPpXPpYNIIT7h8-QenM0Jt5vM.woff2) format('woff2');
    ///     unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
    /// }
    /// ```
    /// The url of the font is located at the seventh line, after "url(".
    pub async fn load(url: &str) -> Result<Font, JsValue> {
        use web_sys::FontFace;
        use web_sys::window;
        use wasm_bindgen_futures::JsFuture;

        let window = window().unwrap();

        let crypto = window.crypto()?;
        let mut randoms = [0; 25];
        crypto.get_random_values_with_u8_array(&mut randoms)?;
        let mut family_name = String::new();
        for random in randoms.iter() {
            family_name.push(match random % 26 {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                7 => 'h',
                8 => 'i',
                9 => 'j',
                10 => 'k',
                11 => 'l',
                12 => 'm',
                13 => 'n',
                14 => 'o',
                15 => 'p',
                16 => 'q',
                17 => 'r',
                18 => 's',
                19 => 't',
                20 => 'u',
                21 => 'v',
                22 => 'w',
                23 => 'x',
                24 => 'y',
                _ => 'z'
            });
        }
        let font = FontFace::new_with_str(&family_name, &format!("url({})", url))?;
        JsFuture::from(font.load()?).await?;

        window.document().unwrap().fonts().add(&font)?;

        Ok(Font {
            name: family_name
        })
    }

    /// Return the Sherif font.
    pub fn sherif() -> Font {
        Font {
            name: String::from("sherif"),
        }
    }

    /// Return the Arial font.
    pub fn arial() -> Font {
        Font {
            name: String::from("Arial"),
        }
    }

    /// Return the Times New Roman font.
    pub fn times_new_roman() -> Font {
        Font {
            name: String::from("Times New Roman"),
        }
    }

    /// Return the Times font.
    pub fn times() -> Font {
        Font {
            name: String::from("Times"),
        }
    }

    /// Return the Courier New font.
    pub fn courier_new() -> Font {
        Font {
            name: String::from("Courier New"),
        }
    }

    /// Return the Courier font.
    pub fn courier() -> Font {
        Font {
            name: String::from("Courier"),
        }
    }

    /// Return the Verdana font.
    pub fn verdana() -> Font {
        Font {
            name: String::from("Verdana"),
        }
    }

    /// Return the Georgia font.
    pub fn georgia() -> Font {
        Font {
            name: String::from("Georgia"),
        }
    }

    /// Return the Palatino font.
    pub fn palatino() -> Font {
        Font {
            name: String::from("Palatino"),
        }
    }

    /// Return the Garamond font.
    pub fn garamond() -> Font {
        Font {
            name: String::from("Garamond"),
        }
    }

    /// Return the Bookman font.
    pub fn bookman() -> Font {
        Font {
            name: String::from("Bookman"),
        }
    }

    /// Return the Comic Sans Ms font.
    pub fn comic_sans_ms() -> Font {
        Font {
            name: String::from("Comic Sans MS"),
        }
    }

    /// Return the Candara font.
    pub fn candara() -> Font {
        Font {
            name: String::from("Candara"),
        }
    }

    /// Return the Helvetica font.
    pub fn helvetica() -> Font {
        Font {
            name: String::from("Helvetica"),
        }
    }

    /// Return the Arial Black font.
    pub fn arial_black() -> Font {
        Font {
            name: String::from("Arial Black"),
        }
    }

    /// Return the Inpact font.
    pub fn impact() -> Font {
        Font {
            name: String::from("Impact"),
        }
    }
}