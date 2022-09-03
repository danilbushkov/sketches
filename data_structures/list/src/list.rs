


struct Node<T> 
    where T: Clone + PartialOrd
{
    data: T,
    next: Option<Box<Node<T>>>
}



pub struct List<T> 
    where T: Clone + PartialOrd
{
    start: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> List<T> 
    where T: Clone + PartialOrd
{
    pub fn new() -> Self {
        Self {
            start: None,
            size: 0,

        }
    }

    pub fn from(vec: &Vec<T>) -> Self 
        
    {
        let mut list = Self {
            start: None, 
            size: 0,
        };
        
        let mut iter = vec.iter();
        if let Some(n) = iter.next() {
            list.start = Some(Box::new(Node{
                data: n.clone(),
                next: None,
            }));
            list.size += 1;
        }
        let mut tmp = &mut list.start;
        while let Some(n) = iter.next() {
            if let Some(t) = tmp {
                t.next = Some(Box::new(Node{
                    data: n.clone(),
                    next: None,
                }));
                list.size += 1;
                tmp = &mut t.next;
            }
        }
        
        list
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();

        let mut tmp = &self.start;
        while let Some(n) = tmp {
            vec.push(n.data.clone());
            tmp = &n.next;
        }

        vec
    }


    pub fn is_empty(&self) -> bool {
        self.start.is_none()
    }

    pub fn push_front(&mut self, value: T) {
        let node = self.start.take();
        self.start = Some(Box::new(Node{
            data: value,
            next: node,
        }));
        self.size += self.size + 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node = self.start.take();
        if let Some(n) = node {
            self.start = n.next;
            self.size += self.size - 1;
            return Some(n.data);
        }
        
        None
    }

    pub fn back_to_front(&mut self) {
        let mut node = self.start.take();
        let mut next: Option<Box<Node<T>>> = None;
        if let Some(n) = &mut node {
            next = n.next.take();
        }
        while let Some(nt) = &mut next {
            let tmp = nt.next.take();
            nt.next = node.take();
            node = next;
            next = tmp;
        }
        
        self.start = node;
        
    }
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn sort(&mut self) {

        
        self.start = Self::merge_sort(0, self.size as isize - 1, self.start.take()).0;

    }

    fn merge_sort(begin: isize, end: isize, node: Option<Box<Node<T>>>) -> (
        Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
        let mut node = node;
        
        if begin >= end {
            let mut next: Option<Box<Node<T>>> = None;
            if let Some(n) = &mut node {
                next = n.next.take();
            }
            return (node, next);
        }
        let middle = (begin + end) / 2;
        let (mut node_a, next) = Self::merge_sort(begin, middle, node);
        let (mut node_b, next_list) = Self::merge_sort(middle+1, end, next);


        

        let mut n = middle - begin + 1;
        let mut m = end - middle;
        let mut start: Option<Box<Node<T>>> = None;
        let mut end: &mut Option<Box<Node<T>>> = &mut None;
        

        if let Some(a) = node_a {
            if let Some(b) = node_b {
                if a.data <= b.data {
                    let mut tmp = a;
                    node_a = tmp.next.take();
                    start = Some(tmp);
                    end = &mut start;
                    n -= 1;
                    node_b = Some(b);

                } else {
                    let mut tmp = b;
                    node_b = tmp.next.take();
                    start = Some(tmp);
                    end = &mut start;
                    m -= 1;
                    node_a = Some(a);

                }
            } else {
                let mut tmp = a;
                node_a = tmp.next.take();
                start = Some(tmp);
                end = &mut start;
                n -= 1;
            }
            
        }

        while n > 0 && m > 0 {
            if let Some(a) = node_a {
                if let Some(b) = node_b {
                    if a.data <= b.data {
                        if let Some(e) = end {
                            let mut tmp = a;
                            node_a = tmp.next.take();
                            e.next = Some(tmp);
                            end = &mut e.next;
                            node_b = Some(b);
                            n -= 1;
                        } else {
                            
                            node_a = Some(a);
                            node_b = Some(b);
                        }
                    } else {
                        if let Some(e) = end {
                            let mut tmp = b;
                            node_b = tmp.next.take();
                            e.next = Some(tmp);
                            end = &mut e.next;
                            node_a = Some(a);
                            m -= 1;
                        } else {
                            
                            node_a = Some(a);
                            node_b = Some(b);
                        }
                    }
                } else {
                    node_a = Some(a);
                    //panic!("{}", n);
                    //m -= 1;
                }
            }
        }
        
        
        while n > 0 {
            if let Some(mut a) = node_a {
                if let Some(e) = end {
                    node_a = a.next.take();
                    e.next = Some(a);
                    end = &mut e.next;
                    n -= 1;
                } else {
                    node_a = Some(a);
                }
            }
            
        }
    
        while m > 0 {
            if let Some(mut b) = node_b {
                if let Some(e) = end {
                    node_b = b.next.take();
                    e.next = Some(b);
                    end = &mut e.next;
                    m -= 1;
                } else {
                    node_b = Some(b);
                }
            }
            
        }
        (start, next_list)
    }

    
}