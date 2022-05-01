use anyhow::{anyhow, Error, Result};
use reqwest::Url;
use resources::{
    informer::{EventHandler, Informer, ListerWatcher, WsStream},
    models::Response,
    objects::KubeObject,
};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;

use crate::{models::PodUpdate, pod_worker::PodWorker};

mod config;
mod docker;
mod models;
mod pod;
mod pod_manager;
mod pod_worker;
mod volume;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("rKubelet started");

    let lw = ListerWatcher {
        lister: Box::new(|_| {
            Box::pin(async {
                let res = reqwest::get("http://localhost:8080/api/v1/pods")
                    .await?
                    .json::<Response<Vec<(String, String)>>>()
                    .await?;
                let res = res.data.ok_or_else(|| anyhow!("Lister failed"))?;
                Ok::<Vec<(String, String)>, Error>(res)
            })
        }),
        watcher: Box::new(|_| {
            Box::pin(async {
                let url = Url::parse("ws://localhost:8080/api/v1/watch/pods")?;
                let (stream, _) = connect_async(url).await?;
                Ok::<WsStream, Error>(stream)
            })
        }),
    };

    // Create work queue and register event handler closures
    let (tx_add, rx) = mpsc::channel::<PodUpdate>(16);
    let tx_update = tx_add.clone();
    let tx_delete = tx_add.clone();
    let eh = EventHandler {
        add_cls: Box::new(move |new| {
            // TODO: this is not good: tx is copied every time add_cls is called, but I can't find a better way
            let tx_add = tx_add.clone();
            Box::pin(async move {
                let pod: KubeObject = serde_json::from_str(&new)?;
                let message = PodUpdate::Add(pod);
                tx_add.send(message).await?;
                Ok(())
            })
        }),
        update_cls: Box::new(move |(_old, new)| {
            let tx_update = tx_update.clone();
            Box::pin(async move {
                let new_pod: KubeObject = serde_json::from_str(&new)?;
                let message = PodUpdate::Update(new_pod);
                tx_update.send(message).await?;
                Ok(())
            })
        }),
        delete_cls: Box::new(move |old| {
            let tx_delete = tx_delete.clone();
            Box::pin(async move {
                let pod: KubeObject = serde_json::from_str(&old)?;
                let message = PodUpdate::Delete(pod);
                tx_delete.send(message).await?;
                Ok(())
            })
        }),
    };

    // Start the informer
    let informer = Informer::new(lw, eh);
    let informer_handle = tokio::spawn(async move { informer.run().await });

    // Start pod worker
    let pod_worker = PodWorker::new();
    let pod_worker_handle = tokio::spawn(async move { pod_worker.run(rx).await });

    pod_worker_handle.await?.expect("Pod worker failed");
    informer_handle.await?
    // TODO: Gracefully shutdown
}
