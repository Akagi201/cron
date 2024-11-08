use std::{collections::HashMap, fs::File, process::Command};

use eyre::Result;
use serde_derive::Deserialize;
use tokio_cron_scheduler::{Job as SchedJob, JobScheduler};

use crate::{config::Config, log::*};

const DEFAULT_WORKING_DIR: &str = "/tmp";
const DEFAULT_LOG_DIR: &str = "/tmp";

#[derive(Debug, Clone, Deserialize)]
pub struct Job {
  pub command: String,
  pub working_dir: Option<String>,
  pub schedule: String,
  pub envs: HashMap<String, String>,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct CronJob {
  pub log_path: Option<String>,
  pub tg_token: Option<String>,
  pub jobs: HashMap<String, Job>,
}

impl CronJob {
  pub fn from_config(config: &Config) -> CronJob {
    CronJob {
      log_path: config.app.log_path.clone(),
      tg_token: config.app.tg_token.clone(),
      jobs: config.jobs.clone(),
    }
  }
  pub async fn run(&self) -> Result<()> {
    let mut sched = JobScheduler::new().await?;
    for (name, job) in self.jobs.iter() {
      info!("Running job {:?}", name);
      let name = name.clone();
      let job = job.clone();
      let log_path = self.log_path.clone().unwrap_or(DEFAULT_LOG_DIR.to_string());
      sched
        .add(SchedJob::new(job.schedule.clone(), move |_uuid, _l| {
          // log_path + "/" + name + "_stdout.log"
          let stdout_file = format!("{}/{}_stdout.log", log_path, name.clone());
          let stderr_file = format!("{}/{}_stderr.log", log_path, name.clone());
          let stdout = File::create(stdout_file).expect("failed to create stdout file");
          let stderr = File::create(stderr_file).expect("failed to create stderr file");
          let working_dir = job.working_dir.clone().unwrap_or(DEFAULT_WORKING_DIR.to_string());

          let child = Command::new(&job.command)
            .envs(&job.envs)
            .current_dir(&working_dir)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .expect("failed to execute child");
          let output = child.wait_with_output().expect("Failed to read output");
          if output.status.success() {
            info!("Job {:?} finished successfully", name);
          } else {
            error!("Job {:?} failed", name);
          }
        })?)
        .await?;
    }
    sched.shutdown_on_ctrl_c();
    sched.set_shutdown_handler(Box::new(|| {
      Box::pin(async move {
        info!("Scheduler shutdown");
      })
    }));
    sched.start().await?;
    Ok(())
  }
}
