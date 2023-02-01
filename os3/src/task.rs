use crate::sync::UPSafeCell;

mod context;
mod switch;
#[allow(clippy::module_inception)]
mod task;

pub use context::TaskContext;
pub use switch::__switch;
pub use task::{TaskControlBlock, TaskStatus};

struct TaskManagerInner {
    current_task: usize,
}

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

impl TaskManager {
    fn run_first_task(&self) -> ! {
        trace!("method run_first_task");
        let inner = self.inner.exclusive_access();
        println!("{}", inner.current_task);
        unimplemented!()
    }
}

lazy_static::lazy_static! {
    pub static ref TASK_MANAGER:TaskManager = {
        trace!("Init task_manager");
        let num_app = 1;
        TaskManager {
        num_app,
        inner: unsafe{
            UPSafeCell::new(TaskManagerInner{
                current_task:0,
            })
        }
        }
    };
}

pub fn run_first_task() {
    trace!("trap::run_first_task");
    TASK_MANAGER.run_first_task();
}
