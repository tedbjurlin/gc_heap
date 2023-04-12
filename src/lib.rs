#![cfg_attr(not(test), no_std)]

use gc_headers::{Tracer, HeapResult, Pointer, GarbageCollectingHeap, HeapError};

#[derive(Copy, Clone, Debug)]
pub struct CopyingHeap<const HEAP_SIZE: usize, const MAX_BLOCKS: usize> {
    // YOUR CODE HERE
}

impl<const HEAP_SIZE: usize, const MAX_BLOCKS: usize> GarbageCollectingHeap for
    CopyingHeap<HEAP_SIZE, MAX_BLOCKS>
{
    fn new() -> Self {
        todo!("Create the heap");
    }

    fn load(&self, p: Pointer) -> HeapResult<u64> {
        todo!("Load a value from the heap");
    }

    fn store(&mut self, p: Pointer, value: u64) -> HeapResult<()> {
        todo!("Store a value in the heap");
    }

    fn malloc<T: Tracer>(&mut self, num_words: usize, tracer: &T) -> HeapResult<Pointer> {
        todo!("Allocate a new block in the heap, collecting if needed.");
    }
}

impl<const HEAP_SIZE: usize, const MAX_BLOCKS: usize>
    CopyingHeap<HEAP_SIZE, MAX_BLOCKS>
{
    pub fn is_allocated(&self, block: usize) -> bool {
        todo!("Return true if block is allocated, false otherwise");
    }   
            
    pub fn num_allocated_blocks(&self) -> usize {
        todo!("Return the number of blocks allocated and in use");
    }       
            
    pub fn size_of(&self, block: usize) -> usize {
        todo!("Return the number of words in the given block");
    }       

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[derive(Default, Debug)]
    struct TestTracer {
        allocations: VecDeque<Pointer>,
    }

    impl Tracer for TestTracer {
        fn trace(&self, blocks_used: &mut [bool]) {
            for p in self.allocations.iter() {
                blocks_used[p.block_num()] = true;
            }
        }
    }

    impl TestTracer {
        fn matches<const HEAP_SIZE: usize, const MAX_BLOCKS: usize>(
            &self,
            allocator: &CopyingHeap<HEAP_SIZE, MAX_BLOCKS>,
        ) -> bool {
            for p in self.allocations.iter() {
                if !allocator.is_allocated(p.block_num()) || allocator.size_of(p.block_num()) != p.len() {
                    return false;
                }
            }
            true
        }

        fn allocate_next<const HEAP_SIZE: usize, const MAX_BLOCKS: usize>(
            &mut self,
            request: usize,
            allocator: &mut CopyingHeap<HEAP_SIZE, MAX_BLOCKS>,
        ) -> HeapResult<()> {
            match allocator.malloc(request, self) {
                HeapResult::Ok(p) => {
                    self.allocations.push_back(p);
                    HeapResult::Ok(())
                }
                HeapResult::Err(e) => HeapResult::Err(e),
            }
        }

        fn deallocate_next_even(&mut self) {
            if self.allocations.len() >= 2 {
                let popped = self.allocations.pop_front().unwrap();
                self.allocations.pop_front().unwrap();
                self.allocations.push_back(popped);
            }
        }

        fn len(&self) -> usize {
            self.allocations.len()
        }

        fn total_allocated(&self) -> usize {
            self.allocations.iter().map(|p| p.len()).sum()
        }

        fn test_in_bounds<const HEAP_SIZE: usize, const MAX_BLOCKS: usize>(
            &self,
            allocator: &mut CopyingHeap<HEAP_SIZE, MAX_BLOCKS>,
        ) {
            let mut value = 0;
            for p in self.allocations.iter() {
                let len = p.len();
                let mut p = Some(*p);
                for _ in 0..len {
                    let pt = p.unwrap();
                    allocator.store(pt, value);
                    assert_eq!(value, allocator.load(pt).unwrap());
                    value += 1;
                    p = pt.next();
                }
            }

            value = 0;
            for p in self.allocations.iter() {
                let len = p.len();
                let mut p = Some(*p);
                for _ in 0..len {
                    let pt = p.unwrap();
                    assert_eq!(value, allocator.load(pt).unwrap());
                    value += 1;
                    p = pt.next();
                }
            }
        }
    }

    #[test]
    fn it_works() {
        let mut allocator = CopyingHeap::<96, 12>::new();
        let mut tracer = TestTracer::default();
        for request in [2, 10, 4, 8, 6, 12, 6, 24, 4, 8, 2, 8] {
            tracer.allocate_next(request, &mut allocator).unwrap();
        }
        assert_eq!(tracer.len(), allocator.num_allocated_blocks());
        assert!(tracer.matches(&allocator));
        assert_eq!(tracer.total_allocated(), 94);

        match tracer.allocate_next(1, &mut allocator) {
            HeapResult::Ok(_) => panic!("Should be an error!"),
            HeapResult::Err(e) => assert_eq!(e, HeapError::OutOfBlocks),
        }

        tracer.test_in_bounds(&mut allocator);

        for _ in 0..(tracer.len() / 2) {
            tracer.deallocate_next_even();
        }
        assert!(tracer.matches(&allocator));
        assert_eq!(tracer.total_allocated(), 24);

        tracer.test_in_bounds(&mut allocator);

        tracer.allocate_next(4, &mut allocator).unwrap();
        assert_eq!(tracer.len(), allocator.num_allocated_blocks());

        tracer.test_in_bounds(&mut allocator);

        tracer.allocate_next(68, &mut allocator).unwrap();
        assert!(tracer.matches(&allocator));
        assert_eq!(tracer.total_allocated(), 96);

        match tracer.allocate_next(1, &mut allocator) {
            HeapResult::Ok(_) => panic!("Should be an error!"),
            HeapResult::Err(e) => assert_eq!(e, HeapError::OutOfMemory),
        }

        tracer.test_in_bounds(&mut allocator);
    }
}
