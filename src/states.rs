pub trait State: private::Sealed {}

pub struct Up;

impl State for Up {}

impl private::Sealed for Up {}

pub struct Down;

impl State for Down {}

impl private::Sealed for Down {}

mod private {
    pub trait Sealed {}
}
