//! Process management syscalls

use crate::task::{
    exit_current_and_run_next, suspend_current_and_run_next, translated_current_ptr, TaskStatus,
};
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{current_task_info, munmap},
};
use crate::{task::mmap, timer::get_time_us};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *translated_current_ptr(ts) = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

/// start 需要映射的虚存起始地址，要求按页对齐
/// len 映射字节长度，可以为 0
/// port：第 0 位表示是否可读，第 1 位表示是否可写，第 2 位表示是否可执行。其他位无效且必须为 0
/// # Error
/// 1. start 没有按页大小对齐
/// 1. port & !0x7 != 0 (port 其余位必须为0)
/// 1. port & 0x7 = 0 (这样的内存无意义)
/// 1. [start, start + len) 中存在已经被映射的页
/// 1. 物理内存不足
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    mmap(start, len, port)
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    munmap(start, len)
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    unsafe {
        *translated_current_ptr(ti) = current_task_info();
    }
    0
}
