use chrono::*;
enum TimeMask {
    TO_YEAR = 0,
    TO_MONTH = 1,
    TO_DAY = 2,
    TO_HOUR = 3,
    TO_MINUTE = 4,
}
struct FlexTime {
    time: chrono::DateTime<Utc>,
    mask: TimeMask, // means the time is accurate to which level
}

struct FlexInterval (FlexTime, FlexTime);

impl FlexTime {
    fn year(&self) -> i32 {
        self.time.year()
    }
    fn month(&self) -> u32 {
        self.time.month()
    }
    fn day(&self) -> u32 {
        self.time.day()
    }
    fn hour(&self) -> u32 {
        self.time.hour()
    }
    fn minute(&self) -> u32 {
        self.time.minute()
    }
    fn index(&self, index: usize) -> i32 {
        match index {
            0 => self.year(),
            1 => self.month() as i32,
            2 => self.day() as i32,
            3 => self.hour() as i32,
            4 => self.minute() as i32,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl FlexInterval{
    fn includes(&self, time: &FlexTime) -> bool {
        // compare time
        if self.0.is_earlier_than(time) && self.1.is_later_than(time) {
            return true;
        }
        return false;
    } 
    fn intersects(&self, interval: &FlexInterval) -> bool {
        // compare time
        if self.0.is_earlier_than(&interval.1) && self.1.is_later_than(&interval.0) {
            return true;
        }
        if self.0.is_later_than(&interval.1) && self.1.is_earlier_than(&interval.0) { 
            return true;
        }
        return false;
    }
}


impl FlexTime{
    fn is_earlier_than(&self, time: &FlexTime) -> bool {
        // compare time
        for i in 0..4 {
            if self.index(i) <= time.index(i) {
                return true;
            }
        }
        return false;
    }
    fn is_later_than(&self, time: &FlexTime) -> bool {
        // compare time
        for i in 0..4 {
            if self.index(i) >= time.index(i) {
                return true;
            }
        }   
        return false;
    }
    fn equals(&self, time: &FlexTime) -> bool {
        // compare time
        for i in 0..4 {
            if self.index(i) != time.index(i) {
                return false;
            }
        }
        return true;
    }
    
}