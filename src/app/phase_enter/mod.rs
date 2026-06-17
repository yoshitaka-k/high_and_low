mod deal;
mod playing;
mod setup;
mod shuffle;
mod end;

use super::{App, GamePhase};

/// フェーズが変わったときの処理
pub(super) fn dispatch(app: &mut App) {
    match app.current_phase {
        GamePhase::Setup => setup::enter_setup(app),
        GamePhase::Shuffle => shuffle::enter_shuffle(app),
        GamePhase::Deal => deal::enter_deal(app),
        GamePhase::Playing => playing::enter_playing(app),
        GamePhase::End => end::enter_end(app),
        _ => {}
    }
}
