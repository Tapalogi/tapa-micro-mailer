mod message_draft;
mod message_fail;
mod message_sent;
mod message_traits;

pub use message_draft::{MessageDraft, MessageDraftBodyType};
pub use message_fail::{MessageFail, MessageFailType};
pub use message_sent::MessageSent;
pub use message_traits::IJsonSerializable;
