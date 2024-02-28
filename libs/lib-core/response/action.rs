use serde::Serialize;

/// Action to be performed on the client
///```rust
/// use lib_core::action;
/// // ACTION::NONE
/// action!();
/// // ACTION::REDIRECT(String::from("/"))
/// action!(r: "/");
/// // ACTION::ALERT(String::from("Some Message"))
/// action!(a: "Some Message");
/// ```
#[cfg_attr(feature = "dev", derive(PartialEq, Debug))]
#[derive(Serialize)]
pub enum Action {
    #[serde(rename = "redirect")]
    REDIRECT(String),

    NONE,

    #[serde(rename = "alert")]
    ALERT(String),
}

impl Action {
    pub fn is_none(&self) -> bool {
        matches!(self, Action::NONE)
    }
}

// Implent IntoResponse for Action to use action only
