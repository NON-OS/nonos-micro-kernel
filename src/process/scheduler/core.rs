// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::realtime;
use super::runqueue::RunQueue;
use super::task::Task;
use super::types::Scheduler;
use spin::{Mutex, Once};

static RUNQUEUE: Once<Mutex<RunQueue>> = Once::new();

pub(crate) fn get_queue() -> &'static Mutex<RunQueue> {
    RUNQUEUE.call_once(|| Mutex::new(RunQueue::new()))
}

pub(crate) fn pending_task_count() -> usize {
    get_queue().lock().len()
}

// Initialized once on the BSP and read-only thereafter. A `Once` removes the
// former `static mut`, which was a data race the moment a second CPU touched
// scheduler state.
static GLOBAL_SCHEDULER: Once<Scheduler> = Once::new();

pub fn init() {
    GLOBAL_SCHEDULER.call_once(|| Scheduler { running_tasks: 0 });
    get_queue().lock().clear();
    realtime::init();
    super::deadline::init();
}

pub fn get() -> Option<&'static Scheduler> {
    GLOBAL_SCHEDULER.get()
}

pub fn spawn(task: Task) {
    if task.priority == super::task::Priority::Deadline {
        let _ = super::deadline::spawn_deadline(task);
    } else if task.priority == super::task::Priority::RealTime {
        realtime::spawn_realtime(task);
    } else {
        get_queue().lock().push(task);
    }
}

pub fn run() -> ! {
    loop {
        super::deadline::run_deadline_tasks();
        realtime::run_realtime_tasks();

        // Pop under the lock, then release it before running the task. Holding
        // the run-queue lock across task.run() serialized every other CPU for a
        // whole quantum and self-deadlocked any task that spawned or woke
        // another (re-entrant lock on the same CPU).
        let next = get_queue().lock().pop();
        if let Some(mut task) = next {
            task.run();
            if !task.is_complete() {
                get_queue().lock().push(task);
            }
        } else {
            crate::arch::idle_cpu();
        }
    }
}

pub fn enter() -> ! {
    run()
}
