use std::marker::PhantomData;

use sealed::JobStatus;
use uuid::Uuid;

pub struct Job<I, S>
where
    S: JobStatus<I>,
{
    id: Uuid,
    status: S,
    _phantom: PhantomData<I>,
}

impl<I, S> Job<I, S>
where
    S: JobStatus<I>,
{
    fn with_id_status(id: Uuid, status: S) -> Self {
        Job {
            id,
            status,
            _phantom: PhantomData,
        }
    }
}

impl<I> Job<I, Pending<I>> {
    pub fn new(input: I) -> Self {
        Job::with_id_status(Uuid::new_v4(), Pending(input))
    }

    pub fn start(self) -> Job<I, Running> {
        Job::with_id_status(self.id, Running)
    }

    pub fn input(&self) -> &I {
        self.status.input()
    }

    pub fn input_mut(&mut self) -> &mut I {
        self.status.input_mut()
    }
}

impl<I> Job<I, Running> {
    pub fn finish<O>(self, output: O) -> Job<I, Done<O>> {
        Job::with_id_status(self.id, Done(output))
    }
}

impl<I, O> Job<I, Done<O>> {
    pub fn output(&self) -> &O {
        self.status.output()
    }

    pub fn into_output(self) -> O {
        self.status.into_output()
    }
}

pub struct Pending<I>(I);
pub struct Running;
pub struct Done<O>(O);

impl<I> JobStatus<I> for Pending<I> {}
impl<I> JobStatus<I> for Running {}
impl<I, O> JobStatus<I> for Done<O> {}

impl<I> Pending<I> {
    fn input(&self) -> &I {
        &self.0
    }

    fn input_mut(&mut self) -> &mut I {
        &mut self.0
    }
}

impl<O> Done<O> {
    fn output(&self) -> &O {
        &self.0
    }

    fn into_output(self) -> O {
        self.0
    }
}

mod sealed {
    pub trait JobStatus<I> {}
}

#[cfg(test)]
mod tests {
    use super::Job;

    #[test]
    fn test() {
        let mut job = Job::new(42);
        assert_eq!(job.input(), &42);
        assert_eq!(job.input_mut(), &mut 42);
        // assert_eq!(job.output(), None);

        let job = job.start();
        // assert_eq!(job.input(), None);
        // assert_eq!(job.input_mut(), None);
        // assert_eq!(job.output(), None);

        let job = job.finish(43);
        // assert_eq!(job.input(), None);
        // assert_eq!(job.input_mut(), None);
        assert_eq!(job.output(), &43);
        assert_eq!(job.into_output(), 43);
    }
}
