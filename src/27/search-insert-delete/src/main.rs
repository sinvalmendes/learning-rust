// The search-insert-delete problem

// Three kinds of threads share access to a singly-linked list:
// searchers, inserters and deleters. Searchers merely examine the list;
// hence they can execute concurrently with each other. Inserters add
// new items to the end of the list; insertions must be mutually exclusive to preclude two inserters from inserting new items at about
// the same time. However, one insert can proceed in parallel with
// any number of searches. Finally, deleters remove items from anywhere in the list. At most one deleter process can access the list at
// a time, and deletion must also be mutually exclusive with searches and insertions.
use std::collections::LinkedList;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    let mut mutex: Arc<Mutex<LinkedList<u32>>> = Arc::new(Mutex::new(list));

    let mut threads = vec![];
    threads.push(create_searcher_thread(&mut mutex, "ST1"));
    threads.push(create_searcher_thread(&mut mutex, "ST2"));
    threads.push(create_inserter_thread(&mut mutex, "IT1"));
    threads.push(create_inserter_thread(&mut mutex, "IT2"));
    threads.push(create_deleter_thread(&mut mutex, "DT1"));
    threads.push(create_deleter_thread(&mut mutex, "DT2"));

    for thread in threads {
        thread.join().unwrap();
    }

    let list = mutex.clone();
    let locked_list = list.lock().unwrap();
    println!("{:?}", *locked_list)
}

fn create_deleter_thread(
    mutex: &mut Arc<Mutex<LinkedList<u32>>>,
    name: &'static str,
) -> thread::JoinHandle<()> {
    let list = mutex.clone();
    let t = thread::spawn(move || {
        let mut locked_list = list.lock().unwrap();
        println!("Deleter thread {}: {:?}", name, *locked_list);
        locked_list.pop_back();
    });

    return t;
}

fn create_inserter_thread(
    mutex: &mut Arc<Mutex<LinkedList<u32>>>,
    name: &'static str,
) -> thread::JoinHandle<()> {
    let list = mutex.clone();
    let t = thread::spawn(move || {
        let mut locked_list = list.lock().unwrap();
        println!("Inserter thread {}: {:?}", name, *locked_list);
        locked_list.push_back(999);
    });

    return t;
}

fn create_searcher_thread(
    mutex: &mut Arc<Mutex<LinkedList<u32>>>,
    name: &'static str,
) -> thread::JoinHandle<()> {
    let list = mutex.clone();
    let t = thread::spawn(move || {
        let locked_list = list.lock().unwrap();
        println!("Searcher thread {}: {:?}", name, *locked_list);
        for item in locked_list.deref() {
            println!("Searcher thread {}: {}", name, item);
        }
    });

    return t;
}
