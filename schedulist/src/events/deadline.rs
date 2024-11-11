use schedulist::time::*;
use schedulist::events::period::*;
use schedulist::events::event::*;

const NULL_DEADLINE: Deadline = Deadline {
    time: 0,
    task: Task::Null,
};

struct Deadline {
    initialized: bool,
    scheduled: bool,
    
}

impl eventCommon for Deadline {}