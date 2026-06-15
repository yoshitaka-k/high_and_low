use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use color_eyre::Result;
use ratatui::crossterm::event::{
    self,
    Event as CrosstermEvent,
    KeyCode,
    KeyEvent,
    KeyModifiers,
    MouseButton,
    MouseEvent,
    MouseEventKind,
};

/// イベント
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
}

/// イベントハンドラ
#[derive(Debug)]
pub struct EventHandler {
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,

    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// 新しいイベントハンドラを作成する
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();

            thread::spawn(move || {
                let mut last_tick = Instant::now();
                let mut pending_key: Option<KeyEvent> = None;
                let mut pending_mouse: Option<MouseEvent> = None;

                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    // イベントを取得
                    if event::poll(timeout).expect("unable to poll event") {
                        if let Some(ev) = Self::from_crossterm(event::read().expect("unable to read event")) {
                            Self::dispatch(&sender, ev, &mut pending_key, &mut pending_mouse);
                        }

                        // イベントを継続的に処理
                        while event::poll(Duration::ZERO).expect("unable to poll event") {
                            if let Some(ev) = Self::from_crossterm(event::read().expect("unable to read event")) {
                                Self::dispatch(&sender, ev, &mut pending_key, &mut pending_mouse);
                            }
                        }
                    }

                    // 1 tick 経過したときの処理
                    if last_tick.elapsed() >= tick_rate {
                        if let Some(key) = pending_key.take() {
                            sender
                                .send(Event::Key(key))
                                .expect("failed to send terminal event");
                        }
                        if let Some(mouse) = pending_mouse.take() {
                            sender
                                .send(Event::Mouse(mouse))
                                .expect("failed to send terminal event");
                        }
                        sender
                            .send(Event::Tick)
                            .expect("failed to send terminal event");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// クロステルムイベントをイベントに変換する
    fn from_crossterm(event: CrosstermEvent) -> Option<Event> {
        match event {
            CrosstermEvent::Key(e) => Some(Event::Key(e)),
            CrosstermEvent::Mouse(e) => Some(Event::Mouse(e)),
            _ => None,
        }
    }

    /// イベントを処理する
    fn dispatch(sender: &mpsc::Sender<Event>, event: Event, pending_key: &mut Option<KeyEvent>, pending_mouse: &mut Option<MouseEvent>) {
        match event {
            Event::Key(key_event) => {
                if key_event.kind != event::KeyEventKind::Press {
                    return;
                }
                if Self::is_immediate_key(&key_event) {
                    sender
                        .send(Event::Key(key_event))
                        .expect("failed to send terminal event");
                    *pending_key = None;
                } else {
                    *pending_key = Some(key_event);
                }
            }
            Event::Mouse(mouse_event) => {
                if matches!(
                    mouse_event.kind,
                    MouseEventKind::Up(MouseButton::Left) | MouseEventKind::Up(MouseButton::Right)
                ) {
                    if Self::is_immediate_mouse(&mouse_event) {
                        sender
                            .send(Event::Mouse(mouse_event))
                            .expect("failed to send terminal event");
                        *pending_mouse = None;
                    } else {
                        *pending_mouse = Some(mouse_event);
                    }
                } else {
                    *pending_mouse = Some(mouse_event);
                }
            }
            Event::Tick => {}
        }
    }

    /// 終了系は待たずに送る（Tick 間隔に合わせない）
    fn is_immediate_key(key_event: &KeyEvent) -> bool {
        matches!(key_event.code, KeyCode::Esc | KeyCode::Char('q'))
            || matches!(
                (key_event.code, key_event.modifiers),
                (KeyCode::Char('c'), KeyModifiers::CONTROL) | (KeyCode::Char('C'), KeyModifiers::CONTROL)
            )
    }

    /// マウスイベントを終了系は待たずに送る（Tick 間隔に合わせない）
    fn is_immediate_mouse(mouse_event: &MouseEvent) -> bool {
        matches!(mouse_event.kind, MouseEventKind::Up(MouseButton::Left))
    }

    /// イベントを取得する
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
