use std::sync::LazyLock;
use std::env;

pub static BOTTOKEN: LazyLock<String> = LazyLock::new(|| {
    env::var("TOKEN").expect("Need a bot token")
});

pub static FACEITTOKEN: LazyLock<String> = LazyLock::new(|| {
    env::var("FACEIT").expect("Need a faceit API")
});

pub static GUILDID: LazyLock<String> = LazyLock::new(|| {
    env::var("GUILDID").expect("Need a guild ID")
});