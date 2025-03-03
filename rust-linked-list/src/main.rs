struct Node {
    val: i32,
    // Option เผื่อมี null
    // Box = allocate heap (single ownership)
    next: Option<Box<Node>>, 
}

struct LinkedList {
    root: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { root: None }
    }

    // ใช้ &mut เพื่อที่จะ mutate ค่าของตัวเอง โดย reference
    fn add(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val,
            next: None,
        });

        if self.root.is_none() {
            // Some บอกว่าใน Option<?> มีค่าอยู่ ไม่เป็น None
            self.root = Some(new_node);
        } else {
            // curr เป็น pointer ชี้ไปหา root by reference
            let mut curr = &mut self.root;
            // ref mut ทำให้เปลี่ยนแปลงค่าได้ แม้จะมี owner
            // *curr = dereferece ไปหาค่าของ curr node
            while let Some(ref mut node) = *curr {
                if node.next.is_none() {
                    node.next = Some(new_node);
                    break;
                } else {
                    curr = &mut node.next;
                }
            }
        }
    }

    fn print_list(&self) {
        let mut curr = &self.root;
        while let Some(node) = curr {
            print!("{} ", node.val);
            curr = &node.next;
        }
        println!();
    }
}

fn main() {
    let mut ll = LinkedList::new();

    ll.add(1);
    ll.add(3);
    ll.add(5);

    ll.print_list();
}
