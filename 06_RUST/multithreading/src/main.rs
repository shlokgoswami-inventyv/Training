use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct MultiThread {
    id: i32,
    recordAddedTime: String,
    threadId: String,
}

static GLOBAL_COUNTER: AtomicI32 = AtomicI32::new(1);

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn main() {
    let shared_data: Arc<Mutex<HashSet<MultiThread>>> = Arc::new(Mutex::new(HashSet::<MultiThread>::new()));

    {
        let data: Arc<Mutex<HashSet<MultiThread>>> = Arc::clone(&shared_data);
        thread::spawn(move || loop {
            let id = GLOBAL_COUNTER.fetch_add(1, Ordering::Relaxed);

            let record: MultiThread = MultiThread {
                id,
                recordAddedTime: now_secs().to_string(),
                threadId: format!("{:?}", thread::current().id()),
            };
                
            data.lock().unwrap().insert(record);

            thread::sleep(Duration::from_secs(10));
        });
    }

    {
        let data: Arc<Mutex<HashSet<MultiThread>>> = Arc::clone(&shared_data);
        thread::spawn(move || loop {
            {
            let set = data.lock().unwrap();
            println!("----- STATE -----");
            for r in set.iter() {
                println!("{:?}", r);
            }
            }
            thread::sleep(Duration::from_secs(1));
        });
    }

    {
        let data = Arc::clone(&shared_data);
        thread::spawn(move || loop {
            let now = now_secs();
            {
            data.lock().unwrap().retain(|r: &MultiThread| {
                let t: u64 = r.recordAddedTime.parse().unwrap();
                !(r.id % 2 == 0 && now - t > 20)
            });
            }
            thread::sleep(Duration::from_secs(20));
        });
    }

    {
        let data: Arc<Mutex<HashSet<MultiThread>>> = Arc::clone(&shared_data);
        thread::spawn(move || loop {
            let now = now_secs();
            {
            data.lock().unwrap().retain(|r| {
                let t: u64 = r.recordAddedTime.parse().unwrap();
                !(r.id % 2 != 0 && now - t > 20)
            });
            }
            thread::sleep(Duration::from_secs(20));
        });
    }

    {
        let data: Arc<Mutex<HashSet<MultiThread>>> = Arc::clone(&shared_data);
        thread::spawn(move || loop {
            {
            let set = data.lock().unwrap();
            let count: usize = set.iter().filter(|r: &&MultiThread| r.id % 2 == 0).count();
            println!("Even count: {}", count);
            }
            thread::sleep(Duration::from_secs(1));
        });
    }

    {
        let data = Arc::clone(&shared_data);
        thread::spawn(move || loop {
            {
            let set = data.lock().unwrap();
            let count: usize = set.iter().filter(|r: &&MultiThread| r.id % 2 != 0).count();
            println!("Odd count: {}", count);
            }
            thread::sleep(Duration::from_secs(1));
        });
    }

    loop {}
}