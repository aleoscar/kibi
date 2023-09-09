pub struct Size {
    pub width: u16,
    pub height: u16
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let width = crossterm::terminal::size()?.0;
        let height = crossterm::terminal::size()?.1;

        Ok(
            Self {size: Size {
                width: width,
                height: height
            }}
        )
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}