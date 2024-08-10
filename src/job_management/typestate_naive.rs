use std::marker::PhantomData;

use uuid::Uuid;

use self::sealed::JobStatus;

pub struct Job<I, O, S>
where
    S: JobStatus<I, O>,
{
    id: Uuid,
    status: S,
    _phantom: PhantomData<(I, O)>,
}

impl<I, O> Job<I, O, Pending<I>> {
    pub fn new(input: I) -> Self {
        Job::with_id_status(Uuid::new_v4(), Pending(input))
    }

    pub fn start(self) -> Job<I, O, Running> {
        Job::with_id_status(self.id, Running)
    }
}

impl<I, O> Job<I, O, Running> {
    pub fn finish(self, output: O) -> Job<I, O, Done<O>> {
        Job::with_id_status(self.id, Done(output))
    }
}

impl<I, O, S> Job<I, O, S>
where
    S: JobStatus<I, O>,
{
    fn with_id_status(id: Uuid, status: S) -> Self {
        Job {
            id,
            status,
            _phantom: PhantomData,
        }
    }

    pub fn input(&self) -> Option<&I> {
        self.status.input()
    }

    pub fn input_mut(&mut self) -> Option<&mut I> {
        self.status.input_mut()
    }

    pub fn output(&self) -> Option<&O> {
        self.status.output()
    }

    pub fn into_output(self) -> Option<O> {
        self.status.into_output()
    }
}

pub struct Pending<I>(I);
pub struct Running;
pub struct Done<O>(O);

impl<I, O> JobStatus<I, O> for Pending<I> {
    fn input(&self) -> Option<&I> {
        Some(&self.0)
    }

    fn input_mut(&mut self) -> Option<&mut I> {
        Some(&mut self.0)
    }
}

impl<I, O> JobStatus<I, O> for Running {}

impl<I, O> JobStatus<I, O> for Done<O> {
    fn output(&self) -> Option<&O> {
        Some(&self.0)
    }

    fn into_output(self) -> Option<O> {
        Some(self.0)
    }
}

mod sealed {
    pub trait JobStatus<I, O> {
        fn input(&self) -> Option<&I> {
            None
        }

        fn input_mut(&mut self) -> Option<&mut I> {
            None
        }

        fn output(&self) -> Option<&O> {
            None
        }

        fn into_output(self) -> Option<O>
        where
            Self: Sized,
        {
            None
        }
    }
}
