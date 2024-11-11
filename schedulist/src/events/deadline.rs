use schedulist::time::*;
use schedulist::events::period::*;
use schedulist::events::event::*;

const NULL_DEADLINE: Deadline = Deadline {
    time: 0,
    task: Task::Null,
};

struct Deadline {
    time: u64,
    task: Task,
}