use core::fmt::{Debug, Display};

pub trait Error: Debug + Display {
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
