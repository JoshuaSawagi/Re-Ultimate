use super::*;

mod status;
mod ground;

pub fn install() {
    status::install();
    ground::install();
}