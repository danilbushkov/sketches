


struct Node<T> 
    where T: Clone
{
    data: T,
    next: Option<Box<Node<T>>>
}



pub struct List<T> 
    where T: Clone
{
    start: Option<Box<Node<T>>>,
}

impl<T> List<T> 
    where T: Clone
{
    pub fn new() -> Self {
        Self {
            start: None,
            

        }
    }

    pub fn from(vec: &Vec<T>) -> Self 
        
    {
        let mut list = Self {
            start: None, 
            
        };
        
        let mut iter = vec.iter();
        if let Some(n) = iter.next() {
            list.start = Some(Box::new(Node{
                data: n.clone(),
                next: None,
            }));
        }
        let mut tmp = &mut list.start;
        while let Some(n) = iter.next() {
            if let Some(t) = tmp {
                t.next = Some(Box::new(Node{
                    data: n.clone(),
                    next: None,
                }));
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
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node = self.start.take();
        if let Some(n) = node {
            self.start = n.next;
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
    
}