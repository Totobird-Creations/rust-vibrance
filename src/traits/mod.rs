use super::consts::Formatting;
use crate::strings::ColouredString;

mod impls;

pub trait Colourisable {
    fn formatted(self, formatting : Vec<Formatting>) -> ColouredString;
}
