

const NULL_DEADLINE: Deadline = Deadline {
    time: 0,
    task: Task::Null,
};

struct Deadline {
    time: u64,
    task: Task,
}