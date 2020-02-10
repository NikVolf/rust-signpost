
pub fn declare_signpost(id: u32) {
    unsafe { kdebug_signpost(id, 0, 0, 0, 0) }
}

pub fn start_signpost(id: u32) {
    unsafe { kdebug_signpost(id, 0, 0, 0, 0) }
}

pub fn end_signpost(id: u32) {
    unsafe { kdebug_signpost(id, 0, 0, 0, 0) }
}

extern "C" {
    fn kdebug_signpost(code: u32, arg1: usize, arg2: usize, arg3: usize, arg4: usize);
    fn kdebug_signpost_start(code: u32, arg1: usize, arg2: usize, arg3: usize, arg4: usize);
    fn kdebug_signpost_end(code: u32, arg1: usize, arg2: usize, arg3: usize, arg4: usize);
}
