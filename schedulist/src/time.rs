
macro_rules! ARBI {
    () => {
        (-1)
    };
}
struct FlexTime(i32, i32, i32, i32, i32); // year, month, day, hour, minute

impl FlexTime {
    fn year(&self) -> i32 {
        self.0
    }
    fn month(&self) -> i32 {
        self.1
    }
    fn day(&self) -> i32 {
        self.2
    }
    fn hour(&self) -> i32 {
        self.3
    }
    fn minute(&self) -> i32 {
        self.4
    }
    fn index(&self, index: usize) -> i32 {
        match index {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            3 => self.3,
            4 => self.4,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl FlexTime {
    fn match_time (pattern: &FlexTime, target: &FlexTime) -> bool {
        // match time
        for i in 0..4 {
            if pattern.index(i) != ARBI!() && pattern.index(i) != target.index(i) {
                return false;
            }
        }
        return true;
    }
    fn validate_time (time: &FlexTime) -> bool {
        // validate time
        let mut flag = true;
        for i in 0..4{
            if time.index(i) == ARBI!() {
                flag = false;
            }
            else {
                if !flag {
                    return false;
                }
            }
        } // the time arbitary part must be at the end, the multi-period time is not allowed, should be set in RepeatPeriod
        if time.year() < 0 || time.month() < 0 || time.day() < 0 || time.hour() < 0 || time.minute() < 0 {
            return false;
        }
        if time.month() > 12 || time.day() > 31 || time.hour() >= 24 || time.minute() >= 60 {
            return false;
        }
        return true;
    }
    fn validate_a_period (start: &FlexTime, end: &FlexTime) -> bool {
        // validate a period
        if !FlexTime::validate_time(start) || !FlexTime::validate_time(end) {
            return false;
        }
        if start.is_later_than(end) {
            return false;
        }
        return true;
    }
}


impl FlexTime{
    fn is_earlier_than(&self, time: &FlexTime) -> bool {
        // compare time
        
        for i in 0..4 {
            if self.index(i) < time.index(i) {
                return true;
            }
        }
        return false;
    }
    fn is_later_than(&self, time: &FlexTime) -> bool {
        // compare time
        for i in 0..4 {
            if self.index(i) > time.index(i) {
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
    fn is_fine_grained(&self) -> bool {
        return !(self.hour() == ARBI!() && self.minute() == ARBI!());
    }
}