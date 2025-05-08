
pub enum OsiLayer{
    Physical,
    Datalink,
    Network,
    Transport,
    Session,
    Presentation,
    Application
}

impl OsiLayer{
    pub fn as_str(&self) -> &'static str {
        match self {
            OsiLayer::Physical => "physical",
            OsiLayer::Datalink => "datalink",
            OsiLayer::Network => "network",
            OsiLayer::Transport => "transport",
            OsiLayer::Session => "session",
            OsiLayer::Presentation => "presentation",
            OsiLayer::Application => "application",
        }
    }
}