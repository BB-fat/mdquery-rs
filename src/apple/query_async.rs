use anyhow::Result;
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc;
use std::task::{Context, Poll};
use std::thread;

use super::{MDItem, MDQuery};

pub struct MDQueryAsyncResult {
    query: Option<MDQuery>,
    receiver: Option<mpsc::Receiver<Result<Vec<MDItem>>>>,
}

impl Future for MDQueryAsyncResult {
    type Output = Result<Vec<MDItem>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(rx) = &self.receiver {
            if let Ok(result) = rx.try_recv() {
                return Poll::Ready(result);
            }
        }

        if let Some(query) = self.query.take() {
            let waker = cx.waker().clone();
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let result = query.execute();
                let _ = tx.send(result);
                waker.wake();
            });

            self.receiver = Some(rx);
        }

        Poll::Pending
    }
}

impl MDQuery {
    /// Executes the MDQuery asynchronously
    /// 
    /// This method encapsulates the query operation in a Future and executes it in a separate thread.
    /// When the Future is awaited, it returns the query results.
    ///
    /// # Returns
    ///
    /// Returns an MDQueryAsyncResult that implements the Future trait,
    /// with an Output type of Result<Vec<MDItem>>
    ///
    /// # Example
    ///
    /// ```
    /// let query = MDQuery::new("kMDItemFSName = \"Safari.app\"", None, None)?;
    /// let items = query.execute_async().await?;
    /// ```
    pub fn execute_async(self) -> MDQueryAsyncResult {
        MDQueryAsyncResult {
            query: Some(self),
            receiver: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MDQueryScope;
    use std::path::PathBuf;

    use super::*;

    #[tokio::test]
    async fn test_md_query_execute_async() {
        let query = MDQuery::new(
            "kMDItemFSName = \"Safari.app\"",
            Some(vec![MDQueryScope::Custom("/Applications".into())]),
            Some(5),
        )
        .unwrap();

        let items = query.execute_async().await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].path().unwrap(),
            PathBuf::from("/Applications/Safari.app")
        );
    }
}
