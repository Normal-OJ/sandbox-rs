use libnoj::*;

use cgroups_rs::fs::cgroup_builder::*;
use cgroups_rs::fs::*;
use cgroups_rs::*;

pub struct DefaultLinuxJudger {
    cg: Option<Cgroup>,
}

impl Judger for DefaultLinuxJudger {
    fn do_config_judge_process(&mut self, _e: &Env, pid: u32) {
        self.cg = Some(Self::setup_cgroup(_e, pid));
        Self::setup_ns(_e, pid);
    }
    fn get_plugin_name(&self) -> &'static str {
        "default-judger-linux"
    }
}

impl Drop for DefaultLinuxJudger {
    fn drop(&mut self) {
        if let Some(cg) = &self.cg {
            cg.delete().unwrap();
        }
    }
}

impl DefaultLinuxJudger {
    pub fn create_instance() -> Box<dyn Judger + Send + Sync> {
        Box::new(DefaultLinuxJudger { cg: None })
    }
    fn setup_cgroup(e: &Env, pid: u32) -> Cgroup {
        let heir = cgroups_rs::fs::hierarchies::auto();
        println!("{}", e.memory_limit);
        let cg = CgroupBuilder::new("runner")
            .memory()
            .kernel_memory_limit(e.memory_limit as i64)
            .memory_hard_limit(e.memory_limit as i64)
            .done()
            .pid()
            .maximum_number_of_processes(MaxValue::Value(e.max_process.into()))
            .done()
            .devices()
            .device(0, 0, devices::DeviceType::All, false, vec![])
            .done()
            .build(heir)
            .unwrap();
        cg.add_task(CgroupPid { pid: pid.into() }).unwrap();
        cg
    }
    fn setup_ns(e: &Env, pid: u32) {}
}
