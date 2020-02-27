/// A color.
#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl Color {
    /// Create a color with a alpha value of 255
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha: 255
        }
    }

    /// Create a color
    pub fn new_with_alpha(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha
        }
    }

    /// Green color
    pub fn green() -> Color {
        Color {
            red: 0,
            green: 128,
            blue: 0,
            alpha: 255
        }
    }

    /// Yellow color
    pub fn yellow() -> Color {
        Color {
            red: 255,
            green: 255,
            blue: 0,
            alpha: 255
        }
    }

    /// Orange color
    pub fn orange() -> Color {
        Color {
            red: 255,
            green: 165,
            blue: 0,
            alpha: 255
        }
    }

    /// Red color
    pub fn red() -> Color {
        Color {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255
        }
    }

    /// Blue color
    pub fn blue() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 255,
            alpha: 255
        }
    }

    /// Cyan color
    pub fn cyan() -> Color {
        Color {
            red: 0,
            green: 255,
            blue: 255,
            alpha: 255
        }
    }

    /// Grey color
    pub fn grey() -> Color {
        Color {
            red: 128,
            green: 128,
            blue: 128,
            alpha: 255
        }
    }

    /// Pink color
    pub fn pink() -> Color {
        Color {
            red: 255,
            green: 192,
            blue: 203,
            alpha: 255
        }
    }

    /// Purple color
    pub fn purple() -> Color {
        Color {
            red: 128,
            green: 0,
            blue: 128,
            alpha: 255
        }
    }

    /// No color
    pub fn black() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255
        }
    }

    /// Pure white
    pub fn white() -> Color {
        Color {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        let mut color = String::from("rgb(");
        color.push_str(&self.red.to_string());
        color.push_str(",");
        color.push_str(&self.green.to_string());
        color.push_str(",");
        color.push_str(&self.blue.to_string());
        color.push_str(")");

        color
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn color_to_string() {
        use super::Color;

        assert_eq!(&Color::purple().to_string(), "rgb(128,0,128)");
    }
}