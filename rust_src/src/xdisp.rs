use lisp::LispObject;
use threads::ThreadState;
use remacs_macros::lisp_fn;
use remacs_sys::bset_update_mode_line;
use libc::c_void;

/// Mark the current buffer for redisplay.
/// This function may be passed to `add-variable-watcher`.
#[lisp_fn]
fn set_buffer_redisplay(
    _symbol: LispObject,
    _newval: LispObject,
    _op: LispObject,
    _where: LispObject,
) -> LispObject {
    unsafe {
        bset_update_mode_line(ThreadState::current_buffer().as_ptr() as *const _ as
            *const c_void)
    };
    ThreadState::current_buffer().prevent_redisplay_optimizations_p = true;
    LispObject::constant_nil()
}
