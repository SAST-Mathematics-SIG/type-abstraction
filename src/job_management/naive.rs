use std::{fmt::Debug, mem::replace};

use uuid::Uuid;

#[derive(Debug)]
pub struct Job<I, O> {
    id: Uuid,
    status: JobStatus,
    input: Option<I>,
    output: Option<O>,
}

#[derive(Debug)]
pub enum JobStatus {
    Pending,
    Running,
    Done,
}

impl<I, O> Job<I, O> {
    pub fn new(input: I) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: JobStatus::Pending,
            input: Some(input),
            output: None,
        }
    }

    pub fn run(self) -> Self
    where
        I: Debug,
    {
        if let JobStatus::Pending = self.status {
            println!("Running with input {:?}", self.input);
            Self {
                id: self.id,
                status: JobStatus::Running,
                input: None,
                output: None,
            }
        } else {
            panic!("Not supported");
        }
    }

    pub fn finish(self, output: O) -> Self
    where
        O: Debug,
    {
        if let JobStatus::Running = self.status {
            println!("Finished with output {output:?}");
            Self {
                id: self.id,
                status: JobStatus::Done,
                input: None,
                output: Some(output),
            }
        } else {
            panic!("Not supported")
        }
    }

    pub fn input(&self) -> Option<&I> {
        self.input.as_ref()
    }

    pub fn input_mut(&mut self) -> Option<&mut I> {
        self.input.as_mut()
    }

    pub fn set_input(&mut self, input: I) -> Option<I> {
        self.input_mut().map(|i| replace(i, input))
    }

    pub fn output(&self) -> Option<&O> {
        self.output.as_ref()
    }

    pub fn into_output(self) -> Option<O> {
        self.output
    }
}
