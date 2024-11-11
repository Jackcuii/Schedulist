

enum eventState {
    Uninitialized,
    Unscheduled,
    full_filled,
}
// 支持推迟顺延还是取消

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
// 支持 UTC
// 支持对某个Tag的事件做map操作