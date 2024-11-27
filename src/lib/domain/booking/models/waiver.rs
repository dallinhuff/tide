use uuid::Uuid;

/// A [Waiver] is a legal release of liability that a [Participant]
/// must sign before participating in a [Trip].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Waiver {
    pub id: WaiverId,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WaiverId(pub Uuid);
