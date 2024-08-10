use std::mem::replace;

use uuid::Uuid;

#[derive(Debug)]
pub struct Job<I, O> {
    id: Uuid,
    status: JobStatus<I, O>,
}

#[derive(Debug)]
pub enum JobStatus<I, O> {
    Pending(I),
    Running,
    Done(O),
}

impl<I, O> Job<I, O> {
    pub fn new(input: I) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: JobStatus::Pending(input),
        }
    }

    pub fn start(self) -> Option<Job<I, O>> {
        if let JobStatus::Pending(_) = self.status {
            Some(Job {
                id: self.id,
                status: JobStatus::Running,
            })
        } else {
            None
        }
    }

    pub fn input(&self) -> Option<&I> {
        if let JobStatus::Pending(i) = &self.status {
            Some(i)
        } else {
            None
        }
    }

    pub fn input_mut(&mut self) -> Option<&mut I> {
        if let JobStatus::Pending(i) = &mut self.status {
            Some(i)
        } else {
            None
        }
    }

    pub fn set_input(&mut self, input: I) -> Option<I> {
        self.input_mut().map(|i| replace(i, input))
    }

    pub fn output(&self) -> Option<&O> {
        if let JobStatus::Done(o) = &self.status {
            Some(o)
        } else {
            None
        }
    }

    pub fn into_output(self) -> Option<O> {
        if let JobStatus::Done(o) = self.status {
            Some(o)
        } else {
            None
        }
    }
}
