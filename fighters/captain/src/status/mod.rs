use super::*;
use globals::*;
// status script import

mod special_n;
mod special_lw;

pub fn install(agent: &mut Agent) {

    special_n::install(agent);
    special_lw::install(agent);
}