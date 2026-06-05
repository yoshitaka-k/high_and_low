use color_eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

use high_and_low::constants::TICK_RATE_MILLIS;
use high_and_low::{
    app::App,
    tui::Tui,
    event::{Event, EventHandler},
    update::{key_update, mouse_update, tick_update},
};

fn main() -> Result<()> {
    let mut app = App::new();
    app.start();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(TICK_RATE_MILLIS);
    let mut tui = Tui::new(terminal, events);

    tui.enter()?;

    while !app.should_quit {
        match tui.events.next()? {
            Event::Tick => tick_update(&mut app),
            Event::Key(key) => key_update(&mut app, key),
            Event::Mouse(mouse) => mouse_update(&mut app, mouse),
        }
        tui.draw(&mut app)?;
    }

    tui.exit()?;
    Ok(())
}
