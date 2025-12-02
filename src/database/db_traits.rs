use futures::{Stream, StreamExt};
use tiberius::QueryStream;
use std::pin::Pin;

pub trait DBLoad: Sized {
    fn from_row_stream(stream: QueryStream<'_>) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<Self>>> + Send + '_>>;
}