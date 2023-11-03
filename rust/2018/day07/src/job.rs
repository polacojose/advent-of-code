use crate::deptree::DepTree;

#[derive(Debug, PartialEq, Eq, Clone)]
enum WorkerState {
    Idle,
    Working(String),
}

#[derive(Debug)]
struct Worker {
    seconds_left: usize,
    state: WorkerState,
}

impl Worker {
    fn step(&mut self) -> Option<String> {
        if self.seconds_left > 0 {
            self.seconds_left -= 1;
            None
        } else {
            let state = self.state.clone();
            match state {
                WorkerState::Idle => None,
                WorkerState::Working(finished_job) => {
                    self.state = WorkerState::Idle;
                    Some(finished_job.clone())
                }
            }
        }
    }
}

pub struct JobConfig {
    pub dep_tree: DepTree,
    pub workers: usize,
    pub base_seconds: usize,
}

pub struct Job {
    completed: Vec<String>,
    job_seconds: usize,
    current_steps: Vec<(String, Vec<String>)>,
    workers: Vec<Worker>,
    base_seconds: usize,
}

impl Job {
    pub fn new(config: JobConfig) -> Self {
        let mut current_steps = config.dep_tree.ordered_steps();
        current_steps.reverse();

        let mut parents = current_steps
            .iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect::<Vec<_>>();

        let workers = (0..config.workers)
            .map(|_| {
                if let Some((k, _)) = parents.pop() {
                    if let Some(pos) = current_steps.iter().position(|(kk, _)| kk == &k) {
                        current_steps.remove(pos);
                    }

                    Worker {
                        seconds_left: Self::seconds_left(config.base_seconds, &k),
                        state: WorkerState::Working(k.to_owned()),
                    }
                } else {
                    Worker {
                        seconds_left: 0,
                        state: WorkerState::Idle,
                    }
                }
            })
            .collect();

        Self {
            completed: Vec::new(),
            job_seconds: 0,
            current_steps,
            workers,
            base_seconds: config.base_seconds,
        }
    }

    fn steps_available(&self) -> Vec<(String, Vec<String>)> {
        self.current_steps
            .iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect()
    }

    fn step(&mut self) {
        // Progress work
        for w in &mut self.workers {
            if let Some(finished_job) = w.step() {
                self.completed.push(finished_job.clone());
                self.current_steps.iter_mut().for_each(|(_, v)| {
                    if let Some(pos) = v.iter().position(|f| f == &finished_job) {
                        v.remove(pos);
                    }
                });
            }
        }

        // Give available jobs to available workers
        let mut steps_available = self.steps_available();
        let mut taken = Vec::new();
        for w in &mut self.workers {
            match w.state {
                WorkerState::Idle => {
                    if let Some((k, _)) = steps_available.pop() {
                        taken.push(k.to_owned());
                        w.state = WorkerState::Working(k.to_owned());
                        w.seconds_left = Self::seconds_left(self.base_seconds, &k)
                    }
                }
                WorkerState::Working(_) => {}
            }
        }

        // Remove taken jobs from pool
        for t in taken {
            if let Some(pos) = self.current_steps.iter().position(|(k, _)| k == &t) {
                self.current_steps.remove(pos);
            }
        }

        self.job_seconds += 1;
    }

    fn workers_working(&self) -> bool {
        for w in &self.workers {
            match w.state {
                WorkerState::Working(_) => return true,
                _ => {}
            }
        }
        return false;
    }

    pub fn complete_job(&mut self) -> usize {
        while self.workers_working() {
            self.step();
        }
        return self.job_seconds;
    }
}

impl Job {
    fn seconds_left(base_seconds: usize, k: &String) -> usize {
        base_seconds + (k.chars().last().unwrap() as usize - 'A' as usize)
    }

    #[cfg(test)]
    fn workers(&self) -> &Vec<Worker> {
        &self.workers
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::deptree::DepTree;

    use super::*;

    fn test_job() -> Job {
        let dep_tree: DepTree = fs::read_to_string("test-input.txt")
            .unwrap()
            .parse()
            .unwrap();
        Job::new(JobConfig {
            dep_tree,
            workers: 2,
            base_seconds: 0,
        })
    }

    #[test]
    fn workers_initialized() {
        let job = test_job();
        let workers = job.workers();
        assert_eq!(workers[0].state, WorkerState::Working("C".to_owned()));
        assert_eq!(workers[1].state, WorkerState::Idle);
    }

    #[test]
    fn can_change_jobs() {
        let mut job = test_job();
        for _ in 0..3 {
            job.step();
        }
        let workers = job.workers();
        assert_eq!(workers[0].state, WorkerState::Working("A".to_owned()));
        assert_eq!(workers[1].state, WorkerState::Working("F".to_owned()));

        let mut job = test_job();
        for _ in 0..4 {
            job.step();
        }
        let workers = job.workers();
        assert_eq!(workers[0].state, WorkerState::Working("B".to_owned()));
        assert_eq!(workers[1].state, WorkerState::Working("F".to_owned()));

        let mut job = test_job();
        for _ in 0..8 {
            job.step();
        }
        let workers = job.workers();
        assert_eq!(workers[0].state, WorkerState::Working("D".to_owned()));
        assert_eq!(workers[1].state, WorkerState::Working("F".to_owned()));

        let mut job = test_job();
        for _ in 0..14 {
            job.step();
        }
        let workers = job.workers();
        assert_eq!(workers[0].state, WorkerState::Working("E".to_owned()));
        assert_eq!(workers[1].state, WorkerState::Idle);

        let mut job = test_job();
        for _ in 0..15 {
            job.step();
        }
        let workers = job.workers();
        assert_eq!(workers[0].state, WorkerState::Idle);
        assert_eq!(workers[1].state, WorkerState::Idle);
        assert_eq!(job.completed.join(""), "CABFDE");
    }

    #[test]
    fn can_get_seconds_to_complete_job() {
        let mut job = test_job();
        assert_eq!(job.complete_job(), 15);
    }
}
