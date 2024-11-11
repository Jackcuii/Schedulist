
use schedulist::events::*;
use schedulist::time::*;
struct PeriodShortRun {
    // common 
    initialized: bool,
    scheduled: bool,
    tags: Vec<String>,
    priority: i8,
    // specific
    interval: FlexInterval,
    linked_deadline: Deadline,
    linked_longrun: PeriodLongRun
}

impl PeriodShortRun {
    pub fn new(startTime: FlexTime, endTime: FlexTime, linked_deadline: Deadline, linked_longrun: PeriodLongRun, priority: i32) -> PeriodShortRun {
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
            linked_deadline,
            linked_longrun
        }
    }
}

impl eventCommon for PeriodShortRun {}

const NULL_SHORT_RUN: PeriodShortRun = PeriodShortRun {
    initialized: false,
    scheduled: false,
    tags: Vec::new(),
    priority: 0,
    interval: FlexInterval::NULL_INTERVAL,
    linked_deadline: NULL_DEADLINE,
    linked_longrun: NULL_LONG_RUNW
};


/// The main difference of long run and short run is that the short run can be converted into a scheduled event.
/// However, if you schedule a long run, it will create a short run duplicate.
struct PeriodLongRun {
    // common 
    initialized: bool,
    scheduled: bool,
    tags: Vec<String>,
    priority: i8,
    // specific
    interval: FlexInterval,
    linked_deadline: Deadline,
    linked_shortrun: Vec<PeriodShortRun>
}

impl eventCommon for PeriodLongRun {}

const NULL_LONG_RUN: PeriodLongRun = PeriodLongRun {
    initialized: false,
    scheduled: false,
    tags: Vec::new(),
    priority: 0,
    interval: FlexInterval::NULL_INTERVAL,
    linked_deadline: NULL_DEADLINE,
    linked_shortrun: Vec::new()
};