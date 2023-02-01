//! Types related to task management

use crate::{loader::init_app_cx, syscall::TaskInfo};

use super::TaskContext;

#[derive(Copy, Clone, Debug)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    pub info: TaskInfo,
    pub start_time:usize,
}

impl TaskControlBlock {
    pub fn new() -> Self {
        Self {
            task_cx: TaskContext::zero_init(),
            info: TaskInfo::new(),
            start_time:0,
        }
    }
    pub fn init(i: usize) -> Self {
        Self {
            task_cx: TaskContext::goto_restore(init_app_cx(i)),
            info: TaskInfo::init(),
            start_time:0
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
