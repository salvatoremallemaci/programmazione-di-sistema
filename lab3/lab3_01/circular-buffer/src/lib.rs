use std::collections::VecDeque;
use crate::Error::{EmptyBuffer, FullBuffer};

pub struct CircularBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer { buffer: VecDeque::with_capacity(capacity), capacity }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.buffer.len() == self.capacity {
            return Err(FullBuffer)
        }
        self.buffer.push_back(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        let res = self.buffer.pop_front();
        if res.is_some() { return Ok(res.unwrap()); }
        Err(EmptyBuffer)
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(element);
    }
}
