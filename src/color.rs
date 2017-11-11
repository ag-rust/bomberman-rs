pub struct Color {
    raw: [f32; 4],
}

impl Color {
    fn build() -> Builder {
        Builder::new()
    }

    pub fn red() -> Color {
        Self::build().red(1.0).done()
    }

    pub fn green() -> Color {
        Self::build().green(1.0).done()
    }

    pub fn blue() -> Color {
        Self::build().blue(1.0).done()
    }

    pub fn black() -> Color {
        Self::build().done()
    }

    pub fn to_raw(&self) -> [f32; 4] {
        self.raw
    }
}

struct Builder {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl Builder {
    fn new() -> Builder {
        Builder {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    fn red(&mut self, amount: f32) -> &mut Builder {
        self.red = amount;
        self
    }

    fn green(&mut self, amount: f32) -> &mut Builder {
        self.green = amount;
        self
    }

    fn blue(&mut self, amount: f32) -> &mut Builder {
        self.blue = amount;
        self
    }

    fn alpha(&mut self, amount: f32) -> &mut Builder {
        self.alpha = amount;
        self
    }

    fn done(&self) -> Color {
        Color { raw: [self.red, self.green, self.blue, self.alpha] }
    }
}
