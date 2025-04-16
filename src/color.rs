#![allow(unused)]
const RESET: &'static str = "\x1b[0m";
const PREFIX: &'static str = "\x1b[";
const SUFFIX: &'static str = "m";

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ForgroundColor {
    Red = 31,
    Green = 32,
    Blue = 34,
}

impl std::fmt::Display for ForgroundColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{PREFIX}{}{SUFFIX}", *self as u8)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BackgroundColor {
    Red = 41,
    Green = 42,
    Blue = 44,
}

impl std::fmt::Display for BackgroundColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{PREFIX}{}{SUFFIX}", *self as u8)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Attribute {
    Bold = 1,
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{PREFIX}{}{SUFFIX}", *self as u8)
    }
}

#[derive(Debug, Clone)]
pub struct ColoredString {
    fg: Option<ForgroundColor>,
    bg: Option<BackgroundColor>,
    text: String,
    attr: Option<Attribute>,
}

impl ColoredString {
    pub fn fg(mut self, fg: ForgroundColor) -> Self {
        self.fg = Some(fg);
        self
    }

    pub fn bg(mut self, bg: BackgroundColor) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn attr(mut self, attr: Attribute) -> Self {
        self.attr = Some(attr);
        self
    }

    pub fn fg_red(mut self) -> Self {
        self.fg = Some(ForgroundColor::Red);
        self
    }

    pub fn fg_green(mut self) -> Self {
        self.fg = Some(ForgroundColor::Green);
        self
    }

    pub fn fg_blue(mut self) -> Self {
        self.fg = Some(ForgroundColor::Blue);
        self
    }

    pub fn bold(mut self) -> Self {
        self.attr = Some(Attribute::Bold);
        self
    }
}

impl std::fmt::Display for ColoredString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { fg, bg, text, attr } = &self;
        match (fg, bg, attr) {
            (Some(fg), Some(bg), Some(attr)) => write!(f, "{fg}{bg}{attr}{text}{RESET}"),
            (Some(fg), Some(bg), None) => write!(f, "{fg}{bg}{text}{RESET}"),
            (Some(fg), None, Some(attr)) => write!(f, "{fg}{attr}{text}{RESET}"),
            (Some(fg), None, None) => write!(f, "{fg}{text}{RESET}"),
            (None, Some(bg), Some(attr)) => write!(f, "{bg}{attr}{text}{RESET}"),
            (None, Some(bg), None) => write!(f, "{bg}{text}{RESET}"),
            (None, None, Some(attr)) => write!(f, "{attr}{text}{RESET}"),
            _ => write!(f, "{text}"),
        }
    }
}

pub trait TerminalColor: std::fmt::Display {
    fn color(&self) -> ColoredString {
        ColoredString {
            fg: None,
            bg: None,
            text: self.to_string(),
            attr: None,
        }
    }
}

impl<T> TerminalColor for T where T: std::fmt::Display {}
