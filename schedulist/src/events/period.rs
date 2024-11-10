
use schedulist::events::*;
use schedulist::time::*;
struct PeriodShortRun {
    // common 
    initialized: bool,
    scheduled: bool,
    tags: Vec<String>,
    priority: i8,
    // specific
    startTime: FlexTime,
    endTime: FlexTime,
    linked_deadline: Deadline
}

impl PeriodShortRun {
    pub fn new(startTime: FlexTime, endTime: FlexTime, linked_deadline: Deadline, priority: i32) -> PeriodShortRun {
        let mut init = true;
        let mut sche = true;
        if !FlexTime::validate_time(&startTime) || !FlexTime::validate_time(&endTime) || !FlexTime::validate_a_period(&startTime, &endTime) {
            init = false;
            sche = false;
        }
        else if !(startTime.is_fine_grained() && endTime.is_fine_grained()) {
            init = true;
            sche = false;
        }
        PeriodShortRun {
            initialized: init,
            scheduled: sche,
            tags: Vec::new(),
            priority,
            startTime,
            endTime,
            linked_deadline
        }
    }
}

impl eventCommon for PeriodShortRun {}