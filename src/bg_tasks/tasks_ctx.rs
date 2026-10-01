use crate::types::AppSettings;

use leptos::logging;
use tokio::time::{Duration, Interval, interval};

// How often to perform a metrics pruning in the DB.
const METRICS_PRUNING_FREQ: Duration = Duration::from_secs(60 * 60); // every hour.

// Lower bound for settings-driven periods; `interval` panics on a zero period.
const MIN_TASK_PERIOD: Duration = Duration::from_secs(1);

fn task_interval(period: Duration) -> Interval {
    interval(period.max(MIN_TASK_PERIOD))
}

// App settings and set of intervals used to schedule each of the tasks.
pub struct TasksContext {
    pub node_bin_version_check: Interval,
    pub balances_retrieval: Interval,
    pub metrics_pruning: Interval,
    pub nodes_metrics_polling: Interval,
    pub disks_usage_check: Interval,
    pub app_settings: AppSettings,
}

impl TasksContext {
    pub fn from(settings: AppSettings) -> Self {
        let mut balances_retrieval = task_interval(settings.rewards_balances_retrieval_freq);
        balances_retrieval.reset(); // the task will trigger the first check by itself

        Self {
            node_bin_version_check: task_interval(settings.node_bin_version_polling_freq),
            balances_retrieval,
            metrics_pruning: interval(METRICS_PRUNING_FREQ),
            nodes_metrics_polling: task_interval(settings.nodes_metrics_polling_freq),
            disks_usage_check: task_interval(settings.disks_usage_check_freq),
            app_settings: settings,
        }
    }

    pub fn apply_settings(&mut self, settings: AppSettings) {
        logging::log!("[BgTask] Applying new settings to background tasks: {settings:#?}");

        // helper to create a new interval only if new period differs from current
        let update_interval = |target: &mut Interval, new_period: Duration| {
            let curr_period = target.period();
            let new_period = new_period.max(MIN_TASK_PERIOD);
            if new_period != curr_period {
                *target = interval(new_period);
                // reset interval to start next period from this instant
                target.reset();
            }
        };

        update_interval(
            &mut self.node_bin_version_check,
            settings.node_bin_version_polling_freq,
        );
        update_interval(
            &mut self.balances_retrieval,
            settings.rewards_balances_retrieval_freq,
        );
        update_interval(
            &mut self.nodes_metrics_polling,
            settings.nodes_metrics_polling_freq,
        );
        update_interval(&mut self.disks_usage_check, settings.disks_usage_check_freq);
        self.app_settings = settings;
    }
}
