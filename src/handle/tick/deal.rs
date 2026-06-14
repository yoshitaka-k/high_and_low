use crate::app::App;

/// ディールフェーズの処理
pub(crate) fn tick_deal(app: &mut App) {
    if let Some(card) = app.game.deck_mut().draw() {
        if app.current == 0 {
            app.game.set_dealer_card(Some(card));
        } else {
            app.game.set_player_card(Some(card));
        }
    }

    app.current = (app.current + 1) % 2;

    if app.game.dealer_card().is_some() && app.game.player_card().is_some() {
        app.advance_phase();
    }
}
