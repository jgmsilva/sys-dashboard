use std::fmt;
#[derive(Debug, Clone, Copy)]
pub enum Panel {
    System,
    Process,
    Files,
    Memory,
    Terminal,
}

impl fmt::Display for Panel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Panel::System => write!(f, "System"),
            Panel::Process => write!(f, "Process"),
            Panel::Files => write!(f, "Files"),
            Panel::Memory => write!(f, "Memory"),
            Panel::Terminal => write!(f, "Terminal"),
        }
    }
}

/*impl Panel {
    fn color(&self) -> {
        match self {
            Panel::System =>
        }
    }
}*/
