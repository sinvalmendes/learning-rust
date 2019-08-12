// The search-insert-delete problem

// Three kinds of threads share access to a singly-linked list:
// searchers, inserters and deleters. Searchers merely examine the list;
// hence they can execute concurrently with each other. Inserters add
// new items to the end of the list; insertions must be mutually exclusive to preclude two inserters from inserting new items at about
// the same time. However, one insert can proceed in parallel with
// any number of searches. Finally, deleters remove items from anywhere in the list. At most one deleter process can access the list at
// a time, and deletion must also be mutually exclusive with searches and insertions.
extern crate rand;

use rand::Rng;
use std::collections::LinkedList;
use std::ops::Deref;
use std::sync::{Arc, Mutex, RwLock};
use std::{thread, time};

fn main() {
    let mut mutex_list: LinkedList<u32> = LinkedList::new();
    mutex_list.push_back(1);
    mutex_list.push_back(2);
    mutex_list.push_back(3);

    let mut mutex: Arc<Mutex<LinkedList<u32>>> = Arc::new(Mutex::new(mutex_list));
    let mut rw_list: LinkedList<i32> = LinkedList::new();
    rw_list.push_back(-1);
    rw_list.push_back(-2);
    rw_list.push_back(-3);
    let mut rw_lock = Arc::new(RwLock::new(rw_list));

    let mut threads = vec![];
    threads.push(create_thread_mutex(&mut mutex, "ST1", "searcher"));
    threads.push(create_thread_mutex(&mut mutex, "ST2", "searcher"));
    threads.push(create_thread_mutex(&mut mutex, "IT1", "inserter"));
    threads.push(create_thread_mutex(&mut mutex, "IT2", "inserter"));
    threads.push(create_thread_mutex(&mut mutex, "DT1", "deleter"));
    threads.push(create_thread_mutex(&mut mutex, "DT2", "deleter"));

    threads.push(create_thread_rw(&mut rw_lock, "ST1", "searcher"));
    threads.push(create_thread_rw(&mut rw_lock, "ST2", "searcher"));
    threads.push(create_thread_rw(&mut rw_lock, "IT1", "inserter"));
    threads.push(create_thread_rw(&mut rw_lock, "IT2", "inserter"));
    threads.push(create_thread_rw(&mut rw_lock, "DT1", "deleter"));
    threads.push(create_thread_rw(&mut rw_lock, "DT2", "deleter"));

    for thread in threads {
        thread.join().unwrap();
    }

    let mutex_list = mutex.clone();
    let mutext_locked_list = mutex_list.lock().unwrap();
    println!("{:?}", *mutext_locked_list);

    let rw_list = rw_lock.clone();
    let rw_locked_list = rw_list.read().unwrap();
    println!("{:?}", *rw_locked_list);
}

fn delete(
    locked_list: &mut std::sync::MutexGuard<'_, std::collections::LinkedList<u32>>,
    name: &'static str,
) {
    println!("Deleter thread {}: {:?}", name, *locked_list);
    locked_list.pop_back();
}

fn search(locked_list: &std::collections::LinkedList<u32>, name: &'static str) {
    for item in locked_list {
        println!("Searcher thread {}: {}", name, item);
    }
}

fn insert(
    locked_list: &mut std::sync::MutexGuard<'_, std::collections::LinkedList<u32>>,
    name: &'static str,
) {
    println!("Inserter thread {}: {:?}", name, *locked_list);
    locked_list.push_back(999);
}

fn create_thread_mutex(
    mutex: &mut Arc<Mutex<LinkedList<u32>>>,
    name: &'static str,
    thread_type: &'static str,
) -> thread::JoinHandle<()> {
    let list = mutex.clone();
    let t = thread::spawn(move || {
        sleep(name);
        let mut locked_list = list.lock().unwrap();
        if thread_type == "searcher" {
            let readable_list = locked_list.deref();
            search(&readable_list, name);
        } else if thread_type == "deleter" {
            delete(&mut locked_list, name);
        } else if thread_type == "inserter" {
            insert(&mut locked_list, name);
        }
    });

    return t;
}

fn create_thread_rw(
    lock: &mut Arc<RwLock<LinkedList<i32>>>,
    name: &'static str,
    thread_type: &'static str,
) -> thread::JoinHandle<()> {
    let list = lock.clone();

    let t = thread::spawn(move || {
        sleep(name);
        if thread_type == "searcher" {
            let readable_list = list.read().unwrap();
            let readable_list = readable_list.deref();
            for item in readable_list {
                println!("Searcher thread {}: {}", name, item);
            }
        } else if thread_type == "deleter" {
            let mut writable_list = list.write().unwrap();
            println!("Deleter thread {}: {:?}", name, *writable_list);
            writable_list.pop_back();
        } else if thread_type == "inserter" {
            let mut writable_list = list.write().unwrap();
            println!("Inserter thread {}: {:?}", name, *writable_list);
            writable_list.push_back(999);
        }
    });

    return t;
}

fn sleep(thread_name: &str) {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1000, 5000);
    println!("{} sleeping for: {} milliseconds", thread_name, random);
    let sleep = time::Duration::from_millis(random);
    thread::sleep(sleep);
}
