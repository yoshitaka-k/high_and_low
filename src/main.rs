use color_eyre::Result;
use ratatui::{Terminal, backend::CrosstermBackend};

use high_and_low::constants::TICK_RATE_MILLIS;
use high_and_low::{
    app::App,
    event::{Event, EventHandler},
    tui::Tui,
    update::{key_update, mouse_update, tick_update},
};

fn main() -> Result<()> {
    let mut app = App::new();
    app.text.header = String::from("Trump game High and Low");

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(TICK_RATE_MILLIS);
    let mut tui = Tui::new(terminal, events);

    tui.enter()?;

    tui.draw(&mut app)?;
    app.ticker_fps.on_frame();

    while !app.should_quit {
        match tui.events.next()? {
            Event::Tick => {
                app.ticker_fps.on_tick();
                tick_update(&mut app);
            }
            Event::Key(key) => key_update(&mut app, key),
            Event::Mouse(mouse) => mouse_update(&mut app, mouse),
        }
        tui.draw(&mut app)?;
        app.ticker_fps.on_frame();
    }

    tui.exit()?;
    Ok(())
}
