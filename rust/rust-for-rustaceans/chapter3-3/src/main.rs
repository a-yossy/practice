use std::mem::ManuallyDrop;

struct FileWriter {
    file: Option<std::fs::File>,
}

impl FileWriter {
    fn new(path: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::File::create(path)?;
        Ok(FileWriter { file: Some(file) })
    }

    async fn close(mut self) -> Result<(), std::io::Error> {
        if let Some(file) = self.file.take() {
            file.sync_all()?;
        }
        Ok(())
    }
}

impl Drop for FileWriter {
    fn drop(&mut self) {
        if let Some(file) = self.file.take() {
            // Handle the file close operation
            let _ = file.sync_all();
        }
    }
}

async fn main_example() {
    let writer = FileWriter::new("test.txt").unwrap();
    if let Err(e) = writer.close().await {
        eprintln!("Error closing file: {}", e);
    }
}

struct UnsafeFileHandler {
    file: ManuallyDrop<std::fs::File>,
}

impl UnsafeFileHandler {
    fn new(path: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::File::create(path)?;
        Ok(UnsafeFileHandler {
            file: ManuallyDrop::new(file),
        })
    }

    unsafe fn close(mut self) -> Result<(), std::io::Error> {
        // `ManuallyDrop::take`はunsafe
        let file = ManuallyDrop::take(&mut self.file);
        // ファイルのクリーンアップ処理
        file.sync_all()?;
        Ok(())
    }
}

impl Drop for UnsafeFileHandler {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.file);
        }
    }
}

struct Grounded;
struct Launched;
struct Rocket<Stage = Grounded> {
    stage: std::marker::PhantomData<Stage>,
}
impl Rocket<Grounded> {
    pub fn launch(self) -> Rocket<Launched> {
        println!("Rocket launched!");
        Rocket {
            stage: std::marker::PhantomData,
        }
    }
}
impl Default for Rocket<Grounded> {
    fn default() -> Self {
        Rocket {
            stage: std::marker::PhantomData,
        }
    }
}
impl Rocket<Launched> {
    pub fn accelerate(&mut self) {
        println!("Rocket accelerating!");
    }
    pub fn decelerate(&mut self) {
        println!("Rocket decelerating!");
    }
}
impl<Stage> Rocket<Stage> {
    pub fn color(&self) {
        println!("Rocket color is red!");
    }
    pub fn weight(&self) {
        println!("Rocket weight is 1000kg!");
    }
}

pub struct Unit {
    pub field: bool,
}

fn is_true(u: Unit) -> bool {
    matches!(u, Unit { field: true })
}

#[non_exhaustive]
pub struct Event {
    pub id: u32,
    pub timestamp: u64,
}

pub trait CanUserCannotImplement: sealed::Sealed {
    fn can_user_implement(&self) -> String;
}

pub trait TraitBound: sealed::Sealed {
    fn trait_bound_method(&self) -> String;
}
mod sealed {
    use crate::TraitBound;

    pub trait Sealed {}
    impl<T> Sealed for T where T: TraitBound {}
}
impl<T> CanUserCannotImplement for T where T: TraitBound {
    fn can_user_implement(&self) -> String {
        self.trait_bound_method()
    }
}

fn is_normal<T: Sized + Send + Sync + Unpin>() {}
#[test]
fn normal_types() {
    is_normal::<i32>();
    is_normal::<String>();
    is_normal::<Vec<i32>>();
}

fn main() {
    let rocket = Rocket::default();
    let mut rocket = rocket.launch();
    rocket.decelerate();

    let event = Event {
        id: 1,
        timestamp: 1622547800,
    };
    match event {
        Event { id, timestamp } => {
            // OK
            println!("Event ID: {}", id);
        }
    }
}
