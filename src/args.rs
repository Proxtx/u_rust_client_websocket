use clap::Parser;

/// Unify client (websocket)
#[derive(Parser, Debug)]
#[clap(author="Proxtx", version="1.0.0", about="Unify client (websocket)", long_about=None)]
pub struct Args {
    /// The name the client uses to identify itself
    #[clap(short, long, value_parser)]
    pub id: String,

    /// Client Key. The client uses this key to authenticate with the server
    #[clap(short, long, value_parser)]
    pub key: String,

    /// Server auth. The client needs this to verify the servers authenticity
    #[clap(short, long, value_parser)]
    pub auth: String,

    /// The client uses this wsUrl to connect to the u_ws_connection_bundler
    #[clap(short, long, value_parser)]
    pub ws_url: String,
}
