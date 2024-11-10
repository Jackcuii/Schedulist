

trait eventCommon {
    // should have the following attributes
    // initialized: bool
    // scheduled: bool
    // tags: Vec<String>
    // priority: int8
    fn is_uninitialized(&self) -> bool {
        return self.initialized == false;
    }
    fn is_unscheduled(&self, event: impl Event) -> bool {
        return self.scheduled == false;
    }
    

}

// 可以配置颜色不同的tag