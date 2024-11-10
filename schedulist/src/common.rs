
use schedulist::FlexTime;

// Three stage: uninitialized -> unscheduled -> full
// 1. uninitialized: only event itself is set, usually is from the auto input
// 2. unscheduled: is not set to a certain time
// 3. full: is set to a certain time

// 显示的界面左面是按时间，右面是一个长的list，包括当天未安排，还有longrun（通配到当天）
// uninit 必须被提醒

struct Calendar {
    full_event_list: Mutex::Vec<Event>,
    unsche_event_list: Mutex::Vec<Event>,
    uninit_event_list: Mutex::Vec<Event>,
}


// 注意区分long run 和short run, short schedule 会修改（也不行），而long run会复制出一个short run


trait ddlCommon {}

impl eventInfoCheck for Event {}

impl Calendar {
    fn add_event(&self, event: impl eventCommon) {
        if event.is_uninitialized() {
            self.uninit_event_list.push(event);
        } 
        else if event.is_unscheduled() {
            self.unsche_event_list.push(event);
        }
        else {
            self.full_event_list.push(event);
        }
    }
    fn remove_event(&self, event: Event) {
        self.event_list.remove(event);
    }
    fn update_event(&self, event: Event) {
        self.event_list.remove(event);
        self.add_event(event);
    }
    fn get_event(&self, time: FlexTime) -> Vec<Event> {
        let mut result: Vec<Event> = Vec::new();
        for event in self.event_list {
            if FlexTime::match_time(event.time, time) {
                result.push(event);
            }
        }
        return result;
    }
}


enum MsgType {
    Update,
}

struct MsgList {
    msg: Vec<String>,
}

MsgList {
    msg: Vec::new(),
} // should be protected by Mutex


struct Event {
    
}

trait FullyInitCheck {
    fn is_fully_initialized(&self) -> bool;
}

impl FullyInitCheck for Event {
    fn is_fully_initialized(&self) -> bool {
        // check if all fields are initialized
    }
}
// 这个地方怎么实现多态？

struct Day {
    is_free: bool,
    event: Vec<Event>,
}

trait dayQuery {
    fn is_free(&self) -> bool;
    fn get_event(&self, day) -> Vec<Event>;
}




trait judgeInCertainDay {
    fn is_free(&self) -> bool;
    fn is_busy(&self) -> bool;
}

trait load_data { 
    fn load(&self);
}

trait save_data {
    fn save(&self);
}

struct Configurations {
    sun_first_day: bool,
}