mod column_type;

use std::pin::Pin;

pub use column_type::ColumnType;

use futures::{Stream, StreamExt};
use tiberius::QueryStream;

pub trait DBLoad: Sized {
    fn from_row_stream(stream: QueryStream<'_>) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<Self>>> + Send + '_>>;
}
