use crate::{loader::get_num_app, sync::UPSafeCell, syscall::TaskInfo, timer::get_time_us};

mod context;
mod switch;
#[allow(clippy::module_inception)]
mod task;

use alloc::vec::Vec;
pub use context::TaskContext;
pub use switch::__switch;
pub use task::{TaskControlBlock, TaskStatus};

struct TaskManagerInner {
    current_task: usize,
    tasks: Vec<TaskControlBlock>,
}

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

impl TaskManager {
    fn run_first_task(&self) -> ! {
        trace!("method run_first_task");
        let mut inner = self.inner.exclusive_access();
        trace!(
            "num_app:{} current_task:{}",
            self.num_app,
            inner.current_task
        );
        let task0 = &mut inner.tasks[0];
        task0.info.status = TaskStatus::Running;
        task0.start_time = get_time_us();
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe { __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr) }
        panic!("Unreachable in method run_first_task");
    }

    fn mark_current_suspend(&self) {
        let mut inner = self.inner.exclusive_access();
        let curr_task_id = inner.current_task;
        inner.tasks[curr_task_id].info.status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let curr_task_id = inner.current_task;
        inner.tasks[curr_task_id].info.status = TaskStatus::Exited;
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let curr_task_id = inner.current_task;
        for i in 0..self.num_app {
            debug!("{:?}", inner.tasks[i].info.status);
        }
        (curr_task_id + 1..=curr_task_id + self.num_app)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].info.status == TaskStatus::Ready)
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let curr_task_id = inner.current_task;
            inner.tasks[next].info.status = TaskStatus::Running;
            inner.current_task = next;
            if inner.tasks[next].start_time == 0 {
                inner.tasks[next].start_time = get_time_us()
            }
            let curr_task_cx_ptr = &mut inner.tasks[curr_task_id].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            unsafe {
                __switch(curr_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            panic!("All applications completed!");
        }
    }

    fn update_task_info_syscall(&self, syscall_id: usize) {
        trace!("method update_task_info");
        let mut inner = self.inner.exclusive_access();
        let curr_task_id = inner.current_task;
        let curr_task = &mut inner.tasks[curr_task_id];
        curr_task.info.syscall_times[syscall_id] += 1;
    }

    /// update time
    fn get_task_info(&self) -> TaskInfo {
        trace!("method get_task_info");
        let mut inner = self.inner.exclusive_access();
        let curr_task_id = inner.current_task;
        let curr_task = &mut inner.tasks[curr_task_id];
        curr_task.info.time = (get_time_us() - curr_task.start_time) / 1000;
        curr_task.info
    }
}

lazy_static::lazy_static! {
    pub static ref TASK_MANAGER:TaskManager = {
        trace!("Init TASK_MANAGER");
        let num_app = get_num_app();
        let mut tasks = Vec::new();
        for i in 0..num_app {
            tasks.push(TaskControlBlock::init(i));
        }
        TaskManager {
        num_app,
        inner: unsafe {
            UPSafeCell::new(TaskManagerInner{
                current_task:0,
                tasks,
            })
        }
        }
    };
}

pub fn run_first_task() {
    trace!("trap::run_first_task");
    TASK_MANAGER.run_first_task();
}

pub fn run_next_task() {
    TASK_MANAGER.run_next_task()
}
pub fn mark_current_suspend() {
    TASK_MANAGER.mark_current_suspend()
}
pub fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited()
}
pub fn suspend_current_and_run_next() {
    mark_current_suspend();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}
pub fn update_task_info_syscall(syscall_id: usize) {
    trace!("trap::update_task_info_syscall");
    TASK_MANAGER.update_task_info_syscall(syscall_id);
}

pub fn get_task_info() -> TaskInfo {
    trace!("trap::get_task_info");
    TASK_MANAGER.get_task_info()
}
