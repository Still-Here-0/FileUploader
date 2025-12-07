use futures::{Stream, StreamExt};
use tiberius::QueryStream;
use std::pin::Pin;

pub trait DBLoad<const N: usize>: Sized {
    const TAB: &'static str;
    const COLS: [&'static str; N];

    fn from_stream(stream: QueryStream<'_>) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<Self>>> + Send + '_>>;
}