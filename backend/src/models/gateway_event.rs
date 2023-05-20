use secrecy::Secret;
use serde::{Deserialize, Serialize};

use super::{
    channel::{Channel, ChannelLike},
    guild::Guild,
    member::{Member, UserLike},
    message::Message,
    snowflake::Snowflake,
    user::{Presence, User},
};

/// A JSON payload that can be received over the websocket by clients.
/// All events are serialized in a way such that they are wrapped in a "data" field.
#[derive(Serialize, Debug, Clone)]
#[non_exhaustive]
#[serde(tag = "event", content = "data", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GatewayEvent {
    /// A chat message.
    MessageCreate(Message),
    /// A peer has joined the chat.
    MemberCreate(Member),
    /// A peer has left the chat.
    MemberRemove(Member),
    /// A guild was created.
    GuildCreate(Guild),
    /// A guild was deleted.
    GuildRemove(Guild),
    /// A channel was created.
    ChannelCreate(Channel),
    /// A channel was deleted.
    ChannelRemove(Channel),
    // A user's presence was updated.
    PresenceUpdate(PresenceUpdatePayload),
    /// The server is ready to accept messages.
    Ready(User),
    /// The server has closed the connection.
    InvalidSession(String),
}

impl EventLike for GatewayEvent {
    fn extract_guild_id(&self) -> Option<Snowflake> {
        match self {
            Self::MessageCreate(message) => message.extract_guild_id(),
            Self::MemberCreate(member) => member.extract_guild_id(),
            Self::MemberRemove(member) => member.extract_guild_id(),
            Self::GuildCreate(guild) => guild.extract_guild_id(),
            Self::GuildRemove(guild) => guild.extract_guild_id(),
            Self::ChannelCreate(channel) => channel.extract_guild_id(),
            Self::ChannelRemove(channel) => channel.extract_guild_id(),
            Self::PresenceUpdate(_) => None,
            Self::Ready(_) => None,
            Self::InvalidSession(_) => None,
        }
    }

    fn extract_user_id(&self) -> Option<Snowflake> {
        match self {
            Self::MessageCreate(message) => message.extract_user_id(),
            Self::MemberCreate(member) => member.extract_user_id(),
            Self::MemberRemove(member) => member.extract_user_id(),
            Self::GuildCreate(guild) => guild.extract_user_id(),
            Self::GuildRemove(guild) => guild.extract_user_id(),
            Self::ChannelCreate(channel) => channel.extract_user_id(),
            Self::ChannelRemove(channel) => channel.extract_user_id(),
            Self::PresenceUpdate(payload) => Some(payload.user_id),
            Self::Ready(user) => user.extract_user_id(),
            Self::InvalidSession(_) => None,
        }
    }
}

pub trait EventLike {
    fn extract_guild_id(&self) -> Option<Snowflake>;
    fn extract_user_id(&self) -> Option<Snowflake>;
}

impl EventLike for Message {
    fn extract_guild_id(&self) -> Option<Snowflake> {
        if let Some(UserLike::Member(member)) = &self.author() {
            Some(member.guild_id())
        } else {
            None
        }
    }
    fn extract_user_id(&self) -> Option<Snowflake> {
        self.author().as_ref().map(|userlike| userlike.id())
    }
}

impl EventLike for Member {
    fn extract_guild_id(&self) -> Option<Snowflake> {
        Some(self.guild_id())
    }
    fn extract_user_id(&self) -> Option<Snowflake> {
        Some(self.user().id())
    }
}

impl EventLike for Channel {
    fn extract_guild_id(&self) -> Option<Snowflake> {
        Some(self.guild_id())
    }
    fn extract_user_id(&self) -> Option<Snowflake> {
        None
    }
}

impl EventLike for Guild {
    fn extract_guild_id(&self) -> Option<Snowflake> {
        Some(self.id())
    }
    fn extract_user_id(&self) -> Option<Snowflake> {
        None
    }
}

impl EventLike for User {
    fn extract_guild_id(&self) -> Option<Snowflake> {
        None
    }
    fn extract_user_id(&self) -> Option<Snowflake> {
        Some(self.id())
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct PresenceUpdatePayload {
    pub user_id: Snowflake,
    pub presence: Presence,
}

/// A JSON payload that can be sent over the websocket by clients.
#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
#[serde(tag = "event", content = "data", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GatewayMessage {
    /// Identify with the server. This should be the first event sent by the client.
    Identify(IdentifyPayload),
}

#[derive(Deserialize, Debug, Clone)]
pub struct IdentifyPayload {
    pub token: Secret<String>,
}
