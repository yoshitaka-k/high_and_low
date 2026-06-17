/// 1 tick の時間（ミリ秒）
pub const TICK_RATE_MILLIS: u64 = 200;

/// フェーズ間の待機 tick 数（200ms × N ≒ 見える間隔）
pub const PHASE_ADVANCE_DELAY_TICKS: u8 = 8;

/// シャッフル表示中のローディングスピナー用文字列
pub const SHUFFLE_SPINNER: [char; 4] = ['|', '/', '-', '\\'];

/// デフォルトのクレジット額
pub const DEFAULT_CREDITS: i32 = 100;

/// デフォルトのベット額
pub const DEFAULT_BET: i32 = 10;

/// 勝利数に応じたボーナスの割合
pub const WIN_STREAK_BONUS: f32 = 0.05;
