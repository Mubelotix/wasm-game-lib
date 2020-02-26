use wasm_bindgen::JsValue;

pub struct Font {
    pub name: String
}

impl Font {
    pub fn sherif() -> Font {
        Font {
            name: String::from("Sherif"),
        }
    }

    pub fn arial() -> Font {
        Font {
            name: String::from("Arial"),
        }
    }

    pub fn times_new_roman() -> Font {
        Font {
            name: String::from("Times New Roman"),
        }
    }

    pub fn times() -> Font {
        Font {
            name: String::from("Times"),
        }
    }

    pub fn courier_new() -> Font {
        Font {
            name: String::from("Courier New"),
        }
    }

    pub fn courier() -> Font {
        Font {
            name: String::from("Courier"),
        }
    }

    pub fn verdana() -> Font {
        Font {
            name: String::from("Verdana"),
        }
    }

    pub fn georgia() -> Font {
        Font {
            name: String::from("Georgia"),
        }
    }

    pub fn palatino() -> Font {
        Font {
            name: String::from("Palatino"),
        }
    }

    pub fn garamond() -> Font {
        Font {
            name: String::from("Garamond"),
        }
    }

    pub fn bookman() -> Font {
        Font {
            name: String::from("Bookman"),
        }
    }

    pub fn comic_sans_ms() -> Font {
        Font {
            name: String::from("Comic Sans MS"),
        }
    }

    pub fn candara() -> Font {
        Font {
            name: String::from("Candara"),
        }
    }

    pub fn helvetica() -> Font {
        Font {
            name: String::from("Helvetica"),
        }
    }

    pub fn arial_black() -> Font {
        Font {
            name: String::from("Arial Black"),
        }
    }

    pub fn impact() -> Font {
        Font {
            name: String::from("Impact"),
        }
    }

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
}