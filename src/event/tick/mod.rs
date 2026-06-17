mod setup;
mod shuffle;
mod deal;
mod playing;
mod result;
mod end;

use crate::app::{App, GamePhase};

pub(crate) fn dispatch(app: &mut App) {
    match app.current_phase {
        GamePhase::Setup => setup::tick_setup(app),
        GamePhase::Shuffle => shuffle::tick_shuffle(app),
        GamePhase::Deal => deal::tick_deal(app),
        GamePhase::Playing => playing::tick_playing(app),
        GamePhase::Result => result::tick_result(app),
        GamePhase::End => end::tick_end(app),
        _ => {}
    }
}
