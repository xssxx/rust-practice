use std::cmp::Ordering;
use std::io::{self, Write};

#[derive(Clone)]
struct HashTable<K, V> {
    items: Vec<(K, V)>,
}

impl<K: Ord, V> HashTable<K, V> {
    fn new() -> Self {
        HashTable { items: Vec::new() }
    }

    fn insert(&mut self, key: K, val: V) {
        self.items.push((key, val));
        self.items.sort_by(|a, b| a.0.cmp(&b.0));
    }

    fn get(&self, key: &K) -> Option<&V> {
        let i = self.binary_search(|x| x.0.cmp(key));

        match i {
            Ok(i) => Some(&self.items[i].1), 
            Err(_) => None,
        }
    }
    
    fn binary_search<F>(&self, mut f: F) -> Result<usize, usize> 
    where 
        F: FnMut(&(K, V)) -> Ordering, 
    {   
        let mut low = 0; 
        let mut high = self.items.len();
        
        while low < high {
            let mid = (low + high) / 2;
            match f(&self.items[mid]) {
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
                Ordering::Equal => return Ok(mid),
            }
        }

        Err(low)
    }
    
}

fn main() {
    let mut h: HashTable<&str, i32> = HashTable::new();

    h.insert("apple", 15);
    h.insert("banana", 20);
    h.insert("giwi", 10);
    h.insert("mango", 99);

    let mut buffer = String::new();
    print!("Enter key to search: ");
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            let key = buffer.trim();

            match h.get(&key) {
                Some(val) => println!("Found: {}", val),
                None => println!("Key not found."),
            }
        }

        Err(e) => panic!("{}", e),
    }
}
