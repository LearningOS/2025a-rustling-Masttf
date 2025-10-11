/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
	where
		T: PartialOrd,
	{
		if list_a.length == 0 {
            return list_b;
        }
        if list_b.length == 0 {
            return list_a;
        }

        let mut result = LinkedList::new();
        let mut a = list_a.start;
        let mut b = list_b.start;

        // 合并两个有序链表
        while a.is_some() && b.is_some() {
            let a_val = unsafe { &(*a.unwrap().as_ptr()).val };
            let b_val = unsafe { &(*b.unwrap().as_ptr()).val };

            if a_val <= b_val {
                // 取 a 链表的节点
                let a_node = a.unwrap();
                a = unsafe { (*a_node.as_ptr()).next };

                // 将节点添加到结果链表
                let node_ptr = Some(a_node);
                unsafe { (*a_node.as_ptr()).next = None };

                match result.end {
                    None => result.start = node_ptr,
                    Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
                }
                result.end = node_ptr;
                result.length += 1;
            } else {
                // 取 b 链表的节点
                let b_node = b.unwrap();
                b = unsafe { (*b_node.as_ptr()).next };

                // 将节点添加到结果链表
                let node_ptr = Some(b_node);
                unsafe { (*b_node.as_ptr()).next = None };

                match result.end {
                    None => result.start = node_ptr,
                    Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
                }
                result.end = node_ptr;
                result.length += 1;
            }
        }

        // 添加剩余的节点
        while let Some(node) = a {
            a = unsafe { (*node.as_ptr()).next };

            let node_ptr = Some(node);
            unsafe { (*node.as_ptr()).next = None };

            match result.end {
                None => result.start = node_ptr,
                Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
            }
            result.end = node_ptr;
            result.length += 1;
        }

        while let Some(node) = b {
            b = unsafe { (*node.as_ptr()).next };

            let node_ptr = Some(node);
            unsafe { (*node.as_ptr()).next = None };

            match result.end {
                None => result.start = node_ptr,
                Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
            }
            result.end = node_ptr;
            result.length += 1;
        }

        // 重要：将原链表的 start 和 end 设置为 None，避免重复释放内存
        // 由于 list_a 和 list_b 被 move 到函数中，它们会在函数结束时被 drop
        // 但是我们已经将它们的节点转移到了 result 中，所以需要避免重复释放
        std::mem::forget(list_a);
        std::mem::forget(list_b);

        result
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}
