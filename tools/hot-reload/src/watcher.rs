// File Watcher
// Überwacht Dateiänderungen im Verzeichnis

use anyhow::Result;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use tokio::sync::oneshot;

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    receiver: Arc<Mutex<mpsc::Receiver<notify::Result<Event>>>>,
    watched_dir: PathBuf,
}

impl FileWatcher {
    pub fn new<P: AsRef<Path>>(directory: P) -> Result<Self> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(tx)?;
        
        let dir = directory.as_ref().to_path_buf();
        watcher.watch(&dir, RecursiveMode::Recursive)?;
        
        Ok(FileWatcher {
            watcher,
            receiver: Arc::new(Mutex::new(rx)),
            watched_dir: dir,
        })
    }
    
    pub async fn wait_for_changes(&self) -> Result<Option<Vec<PathBuf>>> {
        use tokio::task;
        
        let (tx, rx) = oneshot::channel();
        let receiver = Arc::clone(&self.receiver);
        
        task::spawn_blocking(move || {
            loop {
                let recv_result = {
                    let guard = receiver.lock().unwrap();
                    guard.recv()
                };
                
                match recv_result {
                    Ok(Ok(event)) => {
                        if let EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) = event.kind {
                            // Filtere nur .velin Dateien
                            let velin_files: Vec<PathBuf> = event.paths
                                .iter()
                                .filter(|p| {
                                    p.extension()
                                        .and_then(|s| s.to_str())
                                        .map(|s| s == "velin")
                                        .unwrap_or(false)
                                })
                                .cloned()
                                .collect();
                            
                            if !velin_files.is_empty() {
                                let _ = tx.send(Some(velin_files));
                                return;
                            }
                        }
                    }
                    Ok(Err(e)) => {
                        eprintln!("⚠️  Watcher-Fehler: {}", e);
                    }
                    Err(_) => {
                        // Channel geschlossen
                        break;
                    }
                }
            }
            let _ = tx.send(None);
        });
        
        rx.await.map_err(|e| anyhow::anyhow!("Receiver error: {}", e))
    }
}
