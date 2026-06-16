use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serenity::builder::EditMessage;
use serenity::http::Http;
use serenity::model::id::{ChannelId, GuildId, MessageId, UserId};

use super::logger::{LogLevel, Logger};

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// Discord bot configuration, loaded from the `discord` key in `config/General.yaml`.
///
/// # Example YAML
/// ```yaml
/// discord:
///   enabled: true
///   bot_token: "your-bot-token-here"
///   guild_id: 123456789012345678
///   channel_name: "btd6_autoplay"
///   notify_user_id: 987654321098765432   # optional — omit or set to null to disable
///   verbosity: "notice"                  # emerg|alert|crit|error|warn|notice|info|debug|trace
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiscordConfig {
    /// Set to `true` to enable Discord notifications.
    #[serde(default)]
    pub enabled: bool,

    /// Bot token from the Discord Developer Portal.
    #[serde(default)]
    pub bot_token: String,

    /// Numeric ID of the Discord guild (server) the bot lives in.
    #[serde(default)]
    pub guild_id: u64,

    /// Name of the text channel to post updates in (default: `btd6_autoplay`).
    #[serde(default = "DiscordConfig::default_channel_name")]
    pub channel_name: String,

    /// Optional user ID to @-mention on critical notifications (crashes, unexpected stops).
    /// Omit or set to `null` to disable mentions.
    #[serde(default)]
    pub notify_user_id: Option<u64>,

    /// Minimum [`LogLevel`] that triggers a Discord post (default: `notice`).
    /// Messages at or below this level (numerically) are sent to Discord.
    #[serde(default = "DiscordConfig::default_verbosity")]
    pub verbosity: String,
}

impl DiscordConfig {
    fn default_channel_name() -> String {
        "btd6_autoplay".to_string()
    }

    fn default_verbosity() -> String {
        "notice".to_string()
    }
}

impl Default for DiscordConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            bot_token: String::new(),
            guild_id: 0,
            channel_name: Self::default_channel_name(),
            notify_user_id: None,
            verbosity: Self::default_verbosity(),
        }
    }
}

// ---------------------------------------------------------------------------
// Notifier
// ---------------------------------------------------------------------------

/// Sends messages to a Discord channel on behalf of a bot.
///
/// Instantiated once at startup via [`DiscordNotifier::new`] and stored in
/// [`DISCORD_NOTIFIER`].  All public methods are synchronous from the caller's
/// perspective — they block on an internal Tokio runtime.
///
/// # Mutable status message
/// [`DiscordNotifier::update_status`] maintains a single "status" message in
/// the target channel that is edited in-place rather than re-posted, keeping
/// the channel tidy.
pub struct DiscordNotifier {
    http: Http,
    channel_id: ChannelId,
    notify_user_id: Option<UserId>,
    verbosity: LogLevel,
    runtime: tokio::runtime::Runtime,
    /// ID of the persistent wins/losses status message. Edited on every update.
    status_message_id: Mutex<Option<MessageId>>,
}

impl DiscordNotifier {
    /// Build a notifier from `config`, returning `None` when Discord is
    /// disabled, the token is missing, or the channel cannot be located.
    pub fn new(config: &DiscordConfig) -> Option<Self> {
        if !config.enabled {
            return None;
        }
        if config.bot_token.is_empty() {
            Logger::warn("Discord is enabled but `bot_token` is empty; notifications disabled.");
            return None;
        }
        if config.guild_id == 0 {
            Logger::warn("Discord is enabled but `guild_id` is 0; notifications disabled.");
            return None;
        }

        let verbosity = config.verbosity.parse::<LogLevel>().unwrap_or_else(|_| {
            // The default verbosity is "notice"; that same default is used as the
            // fallback here so the two are always consistent.
            Logger::warn(format!(
                "Unknown Discord verbosity '{}'; defaulting to Notice.",
                config.verbosity
            ));
            LogLevel::Notice // matches DiscordConfig::default_verbosity()
        });

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime for Discord notifier");

        let http = Http::new(&config.bot_token);

        // Locate the target channel by name inside the configured guild.
        let guild_id = GuildId::from(config.guild_id);
        let channel_name = config.channel_name.clone();

        let channel_id: Option<ChannelId> = runtime.block_on(async {
            match http.get_channels(guild_id).await {
                Ok(channels) => match channels.iter().find(|c| c.name == channel_name) {
                    Some(c) => Some(c.id),
                    None => {
                        Logger::warn(format!(
                            "Discord channel '{}' not found in guild {}; notifications disabled.",
                            channel_name,
                            config.guild_id
                        ));
                        None
                    }
                },
                Err(e) => {
                    Logger::error(format!(
                        "Failed to fetch Discord guild channels: {:?}",
                        e
                    ));
                    None
                }
            }
        });

        let channel_id = channel_id?;
        let notify_user_id = config.notify_user_id.map(UserId::from);

        Some(Self {
            http,
            channel_id,
            notify_user_id,
            verbosity,
            runtime,
            status_message_id: Mutex::new(None),
        })
    }

    // -----------------------------------------------------------------------
    // Public API
    // -----------------------------------------------------------------------

    /// Post a message if `level` is at or below the configured verbosity threshold.
    ///
    /// Use this for routine operational events (start, stop, round milestones, etc.).
    pub fn notify(&self, level: LogLevel, message: &str) {
        if level > self.verbosity {
            return;
        }
        self.send_message(message);
    }

    /// Post an alert, unconditionally, @-mentioning the configured user (if any).
    ///
    /// Intended for high-severity events such as bot crashes or unexpected exits.
    /// The message is sent regardless of the verbosity setting.
    pub fn alert(&self, message: &str) {
        let content = match self.notify_user_id {
            Some(uid) => format!("<@{}> {}", u64::from(uid), message),
            None => message.to_string(),
        };
        self.send_message(&content);
    }

    /// Edit the persistent wins/losses status message in-place.
    ///
    /// Creates a new message on the first call, or if a previous edit fails
    /// (e.g. the original message was deleted).
    ///
    /// Status updates are high-frequency operational data, so they require a
    /// verbosity level of `Info` or lower (more verbose).  When the threshold
    /// is stricter than `Info` (e.g. `Notice`, `Warn`, …) the update is
    /// silently skipped to avoid spamming the channel.
    pub fn update_status(&self, wins: i32, losses: i32) {
        if LogLevel::Info > self.verbosity {
            return;
        }
        let content = format!(
            "**\u{1f3ae} Status** \u{2014} \u{1f3c6} Wins: **{}** | \u{1f480} Losses: **{}**",
            wins, losses
        );
        let mut id_guard = self.status_message_id.lock().unwrap();

        // Try to edit the existing message first.
        if let Some(msg_id) = *id_guard {
            let result = self.runtime.block_on(self.channel_id.edit_message(
                &self.http,
                msg_id,
                EditMessage::new().content(&content),
            ));
            match result {
                Ok(_) => return,
                Err(e) => {
                    Logger::warn(format!(
                        "Failed to edit Discord status message (will resend): {:?}",
                        e
                    ));
                    *id_guard = None;
                }
            }
        }

        // No prior message or edit failed — post a fresh one and remember its ID.
        match self
            .runtime
            .block_on(self.channel_id.say(&self.http, &content))
        {
            Ok(msg) => *id_guard = Some(msg.id),
            Err(e) => Logger::error(format!(
                "Failed to send Discord status message: {:?}",
                e
            )),
        }
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    fn send_message(&self, content: &str) {
        if let Err(e) = self
            .runtime
            .block_on(self.channel_id.say(&self.http, content))
        {
            Logger::error(format!("Failed to send Discord message: {:?}", e));
        }
    }
}

// ---------------------------------------------------------------------------
// Global singleton
// ---------------------------------------------------------------------------

lazy_static! {
    /// Application-wide Discord notifier.
    ///
    /// Populated once during startup by [`init_discord`].  All helper
    /// functions below are no-ops when this is `None`.
    pub static ref DISCORD_NOTIFIER: Mutex<Option<DiscordNotifier>> = Mutex::new(None);
}

/// Initialise the global [`DISCORD_NOTIFIER`] from the provided config.
///
/// Call this once, early in `main`, after loading `General.yaml`.
pub fn init_discord(config: &DiscordConfig) {
    match DiscordNotifier::new(config) {
        Some(notifier) => {
            *DISCORD_NOTIFIER.lock().unwrap() = Some(notifier);
            Logger::info("Discord notifier initialised.");
        }
        None => Logger::info("Discord notifier not initialised (disabled or config error)."),
    }
}

// ---------------------------------------------------------------------------
// Convenience helpers (call these from main / panic hooks)
// ---------------------------------------------------------------------------

/// Post a Discord notification at the given log level (subject to verbosity).
pub fn discord_notify(level: LogLevel, message: impl AsRef<str>) {
    if let Ok(guard) = DISCORD_NOTIFIER.lock() {
        if let Some(n) = guard.as_ref() {
            n.notify(level, message.as_ref());
        }
    }
}

/// Post a critical Discord alert, @-mentioning the configured user if set.
///
/// Sent unconditionally — use for crashes and unexpected exits.
pub fn discord_alert(message: impl AsRef<str>) {
    if let Ok(guard) = DISCORD_NOTIFIER.lock() {
        if let Some(n) = guard.as_ref() {
            n.alert(message.as_ref());
        }
    }
}

/// Edit (or create) the mutable wins/losses status message in the Discord channel.
pub fn discord_update_status(wins: i32, losses: i32) {
    if let Ok(guard) = DISCORD_NOTIFIER.lock() {
        if let Some(n) = guard.as_ref() {
            n.update_status(wins, losses);
        }
    }
}
