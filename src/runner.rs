use std::{thread, time};
use std::ffi::CStr;
use std::fs::{File, Permissions};
use std::io::{BufWriter, Write};
use std::os::fd::AsRawFd;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::sync::{Arc, Mutex};

use fork::{daemon, Fork};
use nix::libc::{execvp, rusage, STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO, strsignal, timeval, wait4, WEXITSTATUS, WIFEXITED, WTERMSIG};
use nix::sys::resource::Resource::{RLIMIT_AS, RLIMIT_CPU, RLIMIT_FSIZE, RLIMIT_NPROC};
use nix::sys::resource::setrlimit;
use nix::sys::signal;
use nix::sys::signal::{Signal, SIGTERM, SIGXCPU, SIGXFSZ};
use nix::unistd::{dup2, Pid};

use libnoj_rs::Judger;
use libnoj_rs::Env;
use libnoj_rs::Lang;
use libnoj_rs::Context;

struct DefaultJudger;

impl Judger for DefaultJudger {
    fn do_before_run(&mut self, _e: &mut Env) {}
    fn do_in_run(&mut self, _c: Context) {}
    fn do_after_run(&mut self, _e: &mut Env) {}
    fn judge_result(&mut self, _e: &mut Env) {}
    fn is_interactive(&self) -> bool {
        return false;
    }
}

fn create_instance() -> impl Judger {
    DefaultJudger {}
}

fn run_inner(mut judger: impl Judger, mut env: Env) {
    if judger.is_interactive() {
        //TODO: add interactive
    }

    if let Ok(Fork::Parent(pid)) = daemon(true, true) {
        judger.do_before_run(&mut env);
        let mut writer = BufWriter::new(File::options().write(true).open(Path::new(&env.output_file)).unwrap());

        let tle_flag_atomic = Arc::new(Mutex::new(false));
        let tle_flag_atomic_inner = Arc::clone(&tle_flag_atomic);

        let mut usage = rusage {
            ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
            ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
            ru_maxrss: 0,
            ru_ixrss: 0,
            ru_idrss: 0,
            ru_isrss: 0,
            ru_minflt: 0,
            ru_majflt: 0,
            ru_nswap: 0,
            ru_inblock: 0,
            ru_oublock: 0,
            ru_msgsnd: 0,
            ru_msgrcv: 0,
            ru_nsignals: 0,
            ru_nvcsw: 0,
            ru_nivcsw: 0,
            #[cfg(any(target_env = "musl", target_env = "ohos", target_os = "emscripten"))]
            __reserved: [0; 16],
        };

        let mut stat = 0;

        thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(env.runtime_limit + 1000));
            signal::kill(Pid::from_raw(pid), SIGTERM).unwrap();
            let mut tle_flag = tle_flag_atomic_inner.lock().unwrap();
            *tle_flag = true;
        });

        unsafe {
            if wait4(pid, &mut stat, 0, &mut usage) == -1 {
                write!(&mut writer, "RE\nwait4() = -1\n0\n0\n").unwrap();
                return;
            }
        };

        let mut tle_flag = *tle_flag_atomic.lock().unwrap();

        if WIFEXITED(stat) || Signal::try_from(WTERMSIG(stat)).unwrap() == SIGTERM {
            if tle_flag || (((usage.ru_utime.tv_sec * 1000000 + usage.ru_utime.tv_usec) / 1000) as u64 > env.runtime_limit + 2) {
                write!(&mut writer, "TLE\nWEXITSTATUS() = {}\n", WEXITSTATUS(stat)).unwrap();
            } else if (usage.ru_maxrss * 1024) as u64 > env.memory_limit {
                write!(&mut writer, "MLE\nWEXITSTATUS() = {}\n", WEXITSTATUS(stat)).unwrap()
            } else if WEXITSTATUS(stat) != 0 {
                write!(&mut writer, "RE\nWIFEXITED - WEXITSTATUS() = {}\n", WEXITSTATUS(stat)).unwrap();
            } else {
                write!(&mut writer, "Exited Normally\nWIFEXITED - WEXITSTATUS() = {}\n", WEXITSTATUS(stat)).unwrap();
            }
        } else {
            let sig = WTERMSIG(stat);
            let sig_str;
            unsafe {
                sig_str = CStr::from_ptr(strsignal(sig)).to_str().unwrap();
            }
            match Signal::try_from(sig).unwrap() {
                SIGXCPU => {
                    write!(&mut writer, "TLE\nWEXITSTATUS() = {}, WTERMSIG() = {} ({})\n", WEXITSTATUS(stat), sig, sig_str).unwrap();
                    tle_flag = true
                }
                SIGXFSZ => {
                    write!(&mut writer, "OLE\nWEXITSTATUS() = {}, WTERMSIG() = {} ({})\n", WEXITSTATUS(stat), sig, sig_str).unwrap();
                }
                _ => {
                    write!(&mut writer, "RE\nWEXITSTATUS() = {}, WTERMSIG() = {} ({})\n", WEXITSTATUS(stat), sig, sig_str).unwrap();
                }
            }
        }
        if tle_flag {
            write!(&mut writer, "{}\n", env.runtime_limit + 100).unwrap();
        } else {
            write!(&mut writer, "{}\n", (usage.ru_utime.tv_sec * 1000000 + usage.ru_utime.tv_usec) / 1000).unwrap();
            write!(&mut writer, "{}\n", usage.ru_maxrss).unwrap();
        }
        judger.do_after_run(&mut env);
    } else {
        let lang = Lang::try_from(env.lang).unwrap();
        setrlimit(RLIMIT_AS, env.memory_limit * 1024, env.memory_limit * 1024 + 1024).unwrap();
        setrlimit(RLIMIT_FSIZE, env.output_size_limit, env.output_size_limit).unwrap();
        setrlimit(RLIMIT_CPU, env.runtime_limit / 1000 + 1 + if env.runtime_limit % 1000 >= 800 { 0 } else { 1 }
                  , env.runtime_limit / 1000 + 2 + if env.runtime_limit % 1000 >= 800 { 0 } else { 1 }).unwrap();
        setrlimit(RLIMIT_NPROC, (env.max_process + 1) as nix::libc::rlim_t, (env.max_process + 1) as nix::libc::rlim_t).unwrap();

        {
            let input = File::options().read(true).open(Path::new(&env.stdin)).unwrap();
            dup2(input.as_raw_fd(), STDIN_FILENO).unwrap();
        }
        {
            let output = File::options().write(true).create(true).open(Path::new(&env.stdout)).unwrap();
            output.set_permissions(Permissions::from_mode(0o644)).unwrap();
            dup2(output.as_raw_fd(), STDOUT_FILENO).unwrap();
        }
        {
            let err = File::options().write(true).create(true).open(Path::new(&env.stderr)).unwrap();
            err.set_permissions(Permissions::from_mode(0o644)).unwrap();
            dup2(err.as_raw_fd(), STDERR_FILENO).unwrap();
        }

        unsafe {
            execvp(lang.get_execute_argv()[0].as_ptr(),
                   lang.get_execute_argv().iter().map(|m| { m.as_ptr() }).collect::<Vec<_>>().as_ptr());
        }
    }
}

pub fn run(dl_path: String, environment: Env) {
    let judger = create_instance();
    run_inner(judger, environment);
}

