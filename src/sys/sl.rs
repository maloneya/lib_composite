use libc::{c_int, c_uint, c_void};

use super::types::{cycles_t, microsec_t, tcap_prio_t, thdcap_t, thdid_t};

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
pub enum sl_thd_state{
    SL_THD_FREE = 0,
    SL_THD_BLOCKED,
    SL_THD_BLOCKED_TIMEOUT,
    SL_THD_WOKEN,
    SL_THD_RUNNABLE,
    SL_THD_DYING,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct sl_thd {
    pub state: sl_thd_state,
    pub thdid: thdid_t,
    thdcap: thdcap_t,
    prio: tcap_prio_t,
    dependency: *mut sl_thd,

    period: cycles_t,
    periodic_cycs: cycles_t,
    timeout_cycs: cycles_t,
    wakeup_cycs: cycles_t,
    timeout_idx: c_int
}

#[allow(non_camel_case_types)]
pub type cos_thd_fn_t = extern fn(*mut c_void);

#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types, dead_code)]
pub enum sched_param_type_t {
    SCHEDP_NOOP = 0,
    SCHEDP_PRIO,		/* fixed priority */
    SCHEDP_RPRIO,		/* priority relatively higher than current thread */
    /* priority relatively lower (not numerically) than current thread */
    SCHEDP_RLPRIO,
    SCHEDP_DEADLINE,	/* if != window */
    SCHEDP_BUDGET,		/* exec time */
    SCHEDP_WINDOW,     	/* period */
    SCHEDP_PROPORTION,	/* percent required */
    SCHEDP_WEIGHT,		/* proportion compared to other */
    SCHEDP_IDLE, 		/* idle thread: internal use only */
    SCHEDP_INIT, 		/* initialization threads: internal use only */
    SCHEDP_IPI_HANDLER,     /* IPI handler thread: internal use only */
    SCHEDP_TIMER, 		/* timer thread: internal use only */
    SCHEDP_CORE_ID,         /* create the thread on the target core */
    SCHEDP_MAX		/* maximum value */
}

#[allow(non_camel_case_types)]
pub type sched_param_t = u32;

// Provided by the "friend" C file
extern {
    pub fn sched_param_pack_rs(param_type: sched_param_type_t, value: c_uint) -> sched_param_t;
    pub fn sl_thd_curr_rs() -> *mut sl_thd;
}

// From sl.c
#[allow(dead_code)]
extern {
    pub fn sl_init();
    pub fn sl_sched_loop();

    pub fn sl_thd_alloc(fun: cos_thd_fn_t, data: *mut c_void) -> *mut sl_thd;
    pub fn sl_thd_free(t: *mut sl_thd);

    pub fn sl_thd_param_set(t: *mut sl_thd, sp: sched_param_t);

    pub fn sl_thdid() -> thdid_t;

    pub fn sl_thd_lkup(tid: thdid_t) -> *mut sl_thd;

    pub fn sl_cs_enter();
    pub fn sl_cs_exit();

    pub fn sl_thd_block(tid: thdid_t);
    pub fn sl_thd_block_timeout(tid: thdid_t, abs_timeout: cycles_t) -> cycles_t;
    pub fn sl_thd_block_periodic(tid: thdid_t) -> c_uint;

    pub fn sl_thd_wakeup(tid: thdid_t);

    pub fn sl_thd_yield(tid: thdid_t);

    pub fn sl_cyc2usec(cyc: cycles_t) -> microsec_t;
    pub fn sl_usec2cyc(usec: microsec_t) -> cycles_t;

    pub fn sl_now() -> cycles_t;
    pub fn sl_now_usec() -> microsec_t;
}

