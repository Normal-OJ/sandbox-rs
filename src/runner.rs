use std::any::Any;
use std::cell::UnsafeCell;
use std::fs::{File, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::raw::pid_t;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use libnoj::{find_plugin, Env, Judger, Lang};
use tokio::time::sleep;
use tokio::{process::Command, sync::Notify, task};

use crate::default_judger::DefaultLinuxJudger;
use crate::plugin_manager::load_plugin;

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

struct TimeoutThreadBuilder {
    pub notify: Arc<Notify>,
    pub child_pid: Pid,
}

impl TimeoutThreadBuilder {
    fn build(self) -> task::JoinHandle<()> {
        task::spawn(async move {
            self.notify.notified().await;
            println!("Start waiting thread");
            sleep(Duration::from_secs(1)).await;
            kill(self.child_pid, Signal::SIGKILL).unwrap();
        })
    }
}

async fn run_inner(judger: Box<dyn Judger + Send + Sync>, mut env: Env) {
    let notify = Arc::new(Notify::new());

    let judger = Arc::new(std::sync::Mutex::new(judger));

    let args = Lang::try_from(env.lang).unwrap().get_execute_argv();
    let mut cmd = Command::new(args[0]);
    cmd.args(&args[1..]);

    let input = File::options()
        .read(true)
        .open(Path::new(&env.stdin))
        .unwrap();
    cmd.stdin(input);
    let output = File::options()
        .write(true)
        .create(true)
        .open(Path::new(&env.stdout))
        .unwrap();
    output
        .set_permissions(Permissions::from_mode(0o644))
        .unwrap();
    cmd.stdout(output);
    let err = File::options()
        .write(true)
        .create(true)
        .open(Path::new(&env.stderr))
        .unwrap();
    err.set_permissions(Permissions::from_mode(0o644)).unwrap();
    cmd.stderr(err);
    {
        let judger = Arc::clone(&judger);
        unsafe {
            cmd.pre_exec(move || {
                let pid = std::process::id();
                let mut hold = judger.lock().unwrap();
                hold.do_config_judge_process(&env, pid);
                Ok(())
            });
        }
    }
    let mut runner = cmd.spawn().unwrap();
    let monitor_thread = TimeoutThreadBuilder {
        notify: Arc::clone(&notify),
        child_pid: Pid::from_raw(runner.id().unwrap() as i32),
    }
    .build();
    notify.notify_one();
    runner.wait().await.unwrap();

    monitor_thread.abort();
}

pub async fn run(dl_path: String, environment: Env) {
    if !dl_path.is_empty() {
        load_plugin(&dl_path);
    }

    let judger;

    if let Some(jud) = find_plugin() {
        judger = jud;
    } else {
        judger = DefaultLinuxJudger::create_instance();
    }

    run_inner(judger, environment).await;
}
