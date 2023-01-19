use arrayvec::{Array, ArrayVec};

//Cache structure
pub struct LRUCache<A: Array>{

    entries: ArrayVec<A>,
    head: usize,
    tail: usize,
    length: usize,
}

//Entry in cache
pub struct Entry<T>{

        val: T,
        prev: usize,
        next: usize,
}

//default implementation of cache entry
impl<A: Array> Default for LRUCache<A> {

    fn default() -> Self{
        let cache = LRUCache{
            entries: ArrayVec::new(),
            head: 0,
            tail: 0,
            length: 0,
        };
        assert!(
            cache.entries.capacity()< usize::max_value(),
            "Capacity overflow"
        );

        cache
    }
}

struct IterMut<'a, A: 'a + Array>{

    cache: &'a mut LRUCache<A>,
    pos: usize,
    done: bool,
}

impl<'a, T, A> Iterator for IterMut<'a, A>
where 
    T: 'a,
    A: 'a + Array<Item = Entry<T>>,
{
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item>{

        if self.done{
           return None;
        }

        let entry = unsafe{&mut *(&mut self.cache.entries[self.pos] as *mut Entry<T>)};
        let index = self.pos;

        if self.pos == self.cache.tail{
            self.done = true;
        }

        self.pos = entry.next;

        Some((index, &mut entry.val))
    }
}

impl <T, A> LRUCache<A>
where
    A: Array<Item = Entry<T>>,
{
    pub fn len(&self) -> usize{
        self.length
    }

    
    pub fn is_empty(&self) -> bool{
        self.length == 0
    }

    
    fn iter_mut(&mut self) -> IterMut<A>{
        IterMut{
            pos: self.head,
            done: self.is_empty(),
            cache: self,
        }
    }

    
    pub fn clear(&mut self){

        self.entries.clear();
        self.head = 0;
        self.tail = 0;
        self.length = 0;
    }

    
    pub fn front(&self) -> Option<&T>{
        self.entries.get(self.head).map(|e| &e.eval)
    }

    
    pub fn front_mut(&mut self) -> Option<&mut T>{
        self.entries.get_mut(self.head).map(|e| &e.eval)
    }


    fn push_front(&mut self, index: usize){
        if self.entries.len() == 1{
            self.tail = index;
        } else {
            self.entries[index].next = self.head;
            sefl.entried[self.head].prev = index;
        }

        self.head = index;
    }

    fn pop_back(&mut self) -> usize{
        let old_tail = self.tail;
        let new_tail = self.entries[old_tail].prev;
        self.tail = new_tail;
        old_tail
    }

    fn remove(&mut self, index: usize){
        assert!(self.length > 0);

        let prev = self.entries[index].prev;
        let next = self.entries[index].next;

        if index == self.head{
            self.head = next;
        } else{
            self.entries[prev].next = next;
        }

        if index == self.tail{
            self.tail = prev;
        } else{
            self.entries[next].prev = prev;
        }

        self.length -= 1;
    }

    fn touch_index(&mut self, index: usize){
        if index != self.head{
            self.remove(index);

            self.length += 1;
            self.push_front(index);
        }
    }

    
    pub fn touch<F>(&mut self, mut pred: F) -> bool
    where
        F: FnMut(&T) -> bool,
        {
            match self.iter_mut().find(|&(_, ref x|)| pred(x)){
                Some((i,_)) => {
                    self.touch_index(i);
                    true
                },
                None => False,
            }
        }

    
    pub fn lookup<F,T>(&mut self, mut pred: F) -> Option<R>
    where
        F: FnMut(&mut T) -> Option<R>,
    
    {
        let mut result = None;

        for (i, entry) in self.iter_mut(){
            if let Some(r) = pred(entry){
                result = Some((i, r));
                break;
            }
        }

        match result{
            None => None,
            Some((i,r)) => {
                self.touch_index(i);
                Some(r)
            }
        }
    }

    
    pub fn insert(&mut self, val: T){

        let entry = entry{
            val,
            prev: 0,
            next: 0,
        };

        let new head = if self.length == self.entries.capacity(){
            
            let last_index = self.pop_back();
            self.entries[last_index] = entry;

            last_index
        } else{
            self.entries.push(entry);
            self.length += 1;
            self.entries.len() -1
        };

        self.push_front(new_head);
    }
    






}



