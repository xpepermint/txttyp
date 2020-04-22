use ansistr::{TextBackground, TextColor, TextStyle, TextAlign, slice_str,
    clean_str, pad_str, trucate_str, wrap_str, repaire_str, style_str,
    color_str, background_str};

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    text: String,
}

impl Text {

    pub fn new<S: Into<String>>(txt: S) -> Self {
        Self {
            text: txt.into(),
        }
    }

    pub fn bold(mut self) -> Self {
        self.text = style_str(self.text, &TextStyle::Bold);
        self
    }

    pub fn dim(mut self) -> Self {
        self.text = style_str(self.text, &TextStyle::Dim);
        self
    }

    pub fn italic(mut self) -> Self {
        self.text = style_str(self.text, &TextStyle::Italic);
        self
    }

    pub fn underlined(mut self) -> Self {
        self.text = style_str(self.text, &TextStyle::Underlined);
        self
    }

    pub fn blinking(mut self) -> Self {
        self.text = style_str(self.text, &TextStyle::Blinking);
        self
    }

    pub fn inversed(mut self) -> Self {
        self.text = style_str(self.text, &TextStyle::Inversed);
        self
    }

    pub fn hidden(mut self) -> Self {
        self.text = style_str(self.text, &TextStyle::Hidden);
        self
    }
    
    pub fn bgblack(mut self) -> Self {
        self.text = background_str(self.text, &TextBackground::Black);
        self
    }

    pub fn bgred(mut self) -> Self {
        self.text = background_str(self.text, &TextBackground::Red);
        self
    }

    pub fn bggreen(mut self) -> Self {
        self.text = background_str(self.text, &TextBackground::Green);
        self
    }

    pub fn bgyellow(mut self) -> Self {
        self.text = background_str(self.text, &TextBackground::Yellow);
        self
    }

    pub fn bgblue(mut self) -> Self {
        self.text = background_str(self.text, &TextBackground::Blue);
        self
    }

    pub fn bgmagenta(mut self) -> Self {
        self.text = background_str(self.text, &TextBackground::Magenta);
        self
    }

    pub fn bgcyan(mut self) -> Self {
        self.text = background_str(self.text, &TextBackground::Cyan);
        self
    }

    pub fn bgwhite(mut self) -> Self {
        self.text = background_str(self.text, &TextBackground::White);
        self
    }
    
    pub fn black(mut self) -> Self {
        self.text = color_str(self.text, &TextColor::Black);
        self
    }

    pub fn red(mut self) -> Self {
        self.text = color_str(self.text, &TextColor::Red);
        self
    }

    pub fn green(mut self) -> Self {
        self.text = color_str(self.text, &TextColor::Green);
        self
    }

    pub fn yellow(mut self) -> Self {
        self.text = color_str(self.text, &TextColor::Yellow);
        self
    }

    pub fn blue(mut self) -> Self {
        self.text = color_str(self.text, &TextColor::Blue);
        self
    }

    pub fn magenta(mut self) -> Self {
        self.text = color_str(self.text, &TextColor::Magenta);
        self
    }

    pub fn cyan(mut self) -> Self {
        self.text = color_str(self.text, &TextColor::Cyan);
        self
    }

    pub fn white(mut self) -> Self {
        self.text = color_str(self.text, &TextColor::White);
        self
    }

    pub fn clean(mut self) -> Self {
        self.text = clean_str(self.text);
        self
    }

    pub fn append<S: Into<String>>(mut self, txt: S) -> Self {
        self.text = format!("{}{}", self.text, txt.into());
        self
    }

    pub fn prepend<S: Into<String>>(mut self, txt: S) -> Self {
        self.text = format!("{}{}", txt.into(), self.text);
        self
    }

    pub fn slice(mut self, start: usize, end: usize) -> Self {
        self.text = slice_str(self.text, start, end);
        self
    }

    pub fn rpad<S: Into<String>>(mut self, width: usize, chr: S) -> Self {
        self.text = pad_str(self.text, width, &TextAlign::Left, chr);
        self
    }

    pub fn lpad<S: Into<String>>(mut self, width: usize, chr: S) -> Self {
        self.text = pad_str(self.text, width, &TextAlign::Right, chr);
        self
    }

    pub fn cpad<S: Into<String>>(mut self, width: usize, chr: S) -> Self {
        self.text = pad_str(self.text, width, &TextAlign::Center, chr);
        self
    }

    pub fn rtruncate<S: Into<String>>(mut self, width: usize, tail: S) -> Self {
        self.text = trucate_str(self.text, width, &TextAlign::Left, tail);
        self
    }

    pub fn ltruncate<S: Into<String>>(mut self, width: usize, tail: S) -> Self {
        self.text = trucate_str(self.text, width, &TextAlign::Right, tail);
        self
    }

    pub fn ctruncate<S: Into<String>>(mut self, width: usize, tail: S) -> Self {
        self.text = trucate_str(self.text, width, &TextAlign::Center, tail);
        self
    }

    pub fn wrap(mut self, width: usize) -> Self {
        self.text = repaire_str(wrap_str(self.text, width));
        self
    }

    pub fn eol(mut self) -> Self {
        self.text = format!("{}\n", self.text);
        self
    }

    pub fn repair(mut self) -> Self {
        self.text = repaire_str(self.text);
        self
    }

    pub fn to_string(&self) -> String {
        repaire_str(&self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_string() {
        let txt = Text::new("Hello");
        assert_eq!(txt.to_string(), "Hello");
        let txt = txt.red().bgblue();
        assert_eq!(txt.to_string(), "\x1B[44m\x1B[31mHello\x1B[39m\x1B[49m");
        let txt = txt.append(" world").rpad(15, ".");
        assert_eq!(txt.to_string(), "\x1B[44m\x1B[31mHello\x1B[39m\x1B[49m world....");
        let txt = txt.slice(6, 11).prepend("Hello ").wrap(5);
        assert_eq!(txt.to_string(), "Hello\nworld");
    }
}
