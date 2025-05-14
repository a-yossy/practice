use std::{
    fs, io,
    path::PathBuf,
    rc::Rc,
    sync::{
        Arc, Mutex, RwLock,
        atomic::{AtomicBool, AtomicIsize, AtomicUsize, Ordering},
        mpsc,
    },
    thread,
};

use anyhow::Result;
use lazy_static::lazy_static;
use rayon::prelude::*;

struct GigabyteMap;

fn load(filename: &str) -> io::Result<String> {
    std::fs::read_to_string(filename)
}

fn save(filename: &str, text: String) -> io::Result<()> {
    std::fs::write(filename, text)
}

fn process(text: String) -> String {
    text.lines()
        .map(|line| line.to_uppercase())
        .collect::<Vec<_>>()
        .join("\n")
}

fn process_files(filenames: Vec<String>, glossary: &GigabyteMap) -> io::Result<()> {
    for document in filenames {
        let text = load(&document)?;
        let results = process(text);
        save(&document, results)?;
    }

    Ok(())
}

fn split_vec_into_chunks<T>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    let mut chunks = Vec::new();
    let mut current_chunk = Vec::new();

    for item in vec {
        current_chunk.push(item);
        if current_chunk.len() == chunk_size {
            chunks.push(current_chunk);
            current_chunk = vec![];
        }
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    chunks
}

// fn process_files_in_parallel(filenames: Vec<String>, glossary: Arc<GigabyteMap>) -> io::Result<()> {
//     const NTHREADAS: usize = 8;
//     let worklists = split_vec_into_chunks(filenames, NTHREADAS);

//     let mut thread_handles = vec![];
//     for worklist in worklists {
//         let glossary_for_child = glossary.clone();
//         thread_handles.push(thread::spawn(move || {
//             process_files(worklist, &glossary_for_child)
//         }));
//     }

//     for handle in thread_handles {
//         handle.join().unwrap()?;
//     }

//     Ok(())
// }

fn process_file(filename: &str, glossary: &GigabyteMap) -> io::Result<()> {
    let text = load(filename)?;
    let results = process(text);
    save(filename, results)?;
    Ok(())
}

fn process_files_in_parallel(filenames: Vec<String>, glossary: &GigabyteMap) -> io::Result<()> {
    filenames
        .par_iter()
        .map(|filename| process_file(filename, glossary))
        .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 })
        .unwrap_or(Ok(()))
}

fn main() -> Result<()> {
    thread::spawn(|| {
        println!("hello from a child thread");
    });

    process_files_in_parallel(
        vec![
            "file1.txt".to_string(),
            "file2.txt".to_string(),
            "file3.txt".to_string(),
            "file4.txt".to_string(),
        ],
        &GigabyteMap,
    )?;

    let mut pixels = vec![0; 1024 * 1024 * 1024];
    {
        let bands: Vec<(usize, &mut [u8])> = pixels
            .chunks_mut(1024 * 1024 * 1024 / 8)
            .enumerate()
            .collect();
        bands.into_par_iter().for_each(|(i, band)| {
            let top = i;
            let band_bounds = (100, 200);
        });
    }

    let documents: Vec<PathBuf> = vec![
        PathBuf::from("file1.txt"),
        PathBuf::from("file2.txt"),
        PathBuf::from("file3.txt"),
        PathBuf::from("file4.txt"),
    ];

    fn start_file_reader_thread(
        documents: Vec<PathBuf>,
    ) -> (mpsc::Receiver<String>, thread::JoinHandle<io::Result<()>>) {
        let (sender, receiver) = mpsc::channel();
        let handle = thread::spawn(move || {
            for filename in documents {
                let text = fs::read_to_string(filename)?;
                if sender.send(text).is_err() {
                    break;
                }
            }
            Ok(())
        });

        (receiver, handle)
    }

    struct InMemoryIndex;
    impl InMemoryIndex {
        fn new() -> Self {
            InMemoryIndex
        }
    }
    impl InMemoryIndex {
        fn from_single_doucment(doc_id: usize, text: String) -> Self {
            // Simulate creating an index from a single document
            println!("Indexing document {}: {}", doc_id, text);
            InMemoryIndex::new()
        }
    }

    fn start_file_indexing_thread(
        texts: mpsc::Receiver<String>,
    ) -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>) {
        let (sender, receiver) = mpsc::channel();
        let handle = thread::spawn(move || {
            for (doc_id, text) in texts.into_iter().enumerate() {
                let index = InMemoryIndex::from_single_doucment(doc_id, text);
                if sender.send(index).is_err() {
                    break;
                }
            }
        });

        (receiver, handle)
    }

    // let rc1 = Rc::new("ouch".to_string());
    // let rc2 = rc1.clone();
    // thread::spawn(move || {
    //     rc2.clone();
    // });
    // rc1.clone();

    pub trait OffThreadExt: Iterator {
        fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
    }

    impl<T> OffThreadExt for T
    where
        T: Iterator + Send + 'static,
        T::Item: Send + 'static,
    {
        fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
            let (sender, receiver) = mpsc::sync_channel(1024);

            thread::spawn(move || {
                for item in self {
                    if sender.send(item).is_err() {
                        break;
                    }
                }
            });
            receiver.into_iter()
        }
    }

    type PlayerId = u32;
    const GAME_SIZE: usize = 8;
    type WaitingList = Vec<PlayerId>;
    struct FernEmpireApp {
        waiting_list: Mutex<WaitingList>,
    }

    impl FernEmpireApp {
        fn join_waiting_list(&self, player: PlayerId) {
            let mut guard = self.waiting_list.lock().unwrap();
            guard.push(player);
            if guard.len() == GAME_SIZE {
                let players = guard.split_off(0);
                println!("Starting game with players: {:?}", players);
            }
        }
    }

    let app = Arc::new(FernEmpireApp {
        waiting_list: Mutex::new(vec![]),
    });

    pub mod shared_channel {
        use std::sync::mpsc::{Receiver, Sender, channel};
        use std::sync::{Arc, Mutex};

        #[derive(Clone)]
        pub struct SharedReceiver<T>(Arc<Mutex<Receiver<T>>>);

        impl<T> Iterator for SharedReceiver<T> {
            type Item = T;

            fn next(&mut self) -> Option<Self::Item> {
                let guard = self.0.lock().unwrap();
                guard.recv().ok()
            }
        }

        pub fn shared_channel<T>() -> (Sender<T>, SharedReceiver<T>) {
            let (sender, receiver) = channel();
            (sender, SharedReceiver(Arc::new(Mutex::new(receiver))))
        }
    }

    struct AppConfig;
    struct FernEmpireApp2 {
        config: RwLock<AppConfig>,
    }

    impl FernEmpireApp2 {
        fn mushrooms_enabled(&self) -> bool {
            let config_guard = self.config.read().unwrap();

            true
        }

        fn reload_config(&self) -> io::Result<()> {
            let new_config = AppConfig;
            let mut config_guard = self.config.write().unwrap();
            *config_guard = new_config;
            Ok(())
        }
    }

    let atom = AtomicIsize::new(0);
    atom.fetch_add(1, Ordering::SeqCst);

    let cancel_flag = Arc::new(AtomicBool::new(false));
    let worker_cancel_flag = cancel_flag.clone();
    let worker_handle = thread::spawn(move || {
        for pixel in 0..100 {
            if worker_cancel_flag.load(Ordering::SeqCst) {
                return None;
            }
        }

        Some(())
    });
    cancel_flag.store(true, Ordering::SeqCst);
    worker_handle.join().unwrap();

    static PACKETS_SERVED: AtomicUsize = AtomicUsize::new(0);

    struct Color {
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    }
    const fn mono_to_rgba(level: u8) -> Color {
        Color {
            red: level,
            green: level,
            blue: level,
            alpha: 0xFF,
        }
    }

    const WHITE: Color = mono_to_rgba(255);
    const BLACK: Color = mono_to_rgba(000);

    static HOSTNAME: Mutex<String> = Mutex::new(String::new());

    lazy_static! {
        static ref HOSTNAME2: Mutex<String> = Mutex::new(String::new());
    }

    Ok(())
}
