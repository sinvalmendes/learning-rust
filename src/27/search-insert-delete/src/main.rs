// The search-insert-delete problem

// Three kinds of threads share access to a singly-linked list:
// searchers, inserters and deleters. Searchers merely examine the list;
// hence they can execute concurrently with each other. Inserters add
// new items to the end of the list; insertions must be mutually exclusive to preclude two inserters from inserting new items at about
// the same time. However, one insert can proceed in parallel with
// any number of searches. Finally, deleters remove items from anywhere in the list. At most one deleter process can access the list at
// a time, and deletion must also be mutually exclusive with searches and insertions.
use std::collections::LinkedList;
use std::thread;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    let searcher_thread = create_searcher_thread("ST1");
    searcher_thread.join().unwrap();

    println!("{:?}", list);
}

fn create_searcher_thread(name: &'static str) -> thread::JoinHandle<()> {
    let t = thread::spawn(move || {
        println!("Searcher thread {}", name);
    });

    return t;
}
