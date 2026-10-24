use super::*;
use globals::*;

mod special_s;
mod special_lw;

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    special_s::install(agent);
    special_lw::install(agent);
}