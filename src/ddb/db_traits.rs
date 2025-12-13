use futures::{Stream, StreamExt};
use tiberius::QueryStream;
use std::pin::Pin;

pub trait DBLoad: Sized {
    const LEN: usize;
    const TAB: &'_ str;
    const COLS: &'_ [&'_ str];

    fn from_stream(stream: QueryStream<'_>) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<Self>>> + Send + '_>>;
}