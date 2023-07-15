use std::error::Error;
use toml::Table;

#[derive(Clone)]
struct Env {
    pub cwd: String,
    pub large_stack: bool,
    pub max_process: i32,
    pub memory_limit: u64,
    pub output_file: String,
    pub output_size_limit: u64,
    pub runtime_limit: u64,
    pub stderr: String,
    pub stdin: String,
    pub stdout: String
}

impl Env {
    fn new(mut src_str: &str) -> Result<Self, Box<dyn Error>> {
        let table = src_str.parse::<Table>()?;

        Ok(Self {
            cwd: table["cwd"].as_str().unwrap_or("./").to_string(),
            large_stack: table["large-stack"].as_bool().unwrap_or(false),
            max_process: table["max-process"].as_integer().unwrap_or(1) as i32,
            memory_limit: table["memory-limit"].as_integer().unwrap_or(1000000) as u64,
            output_file: table["output"].as_str().unwrap_or("./out").to_string(),
            output_size_limit: table["output-size-limit"].as_integer().unwrap_or(100000) as u64,
            runtime_limit: table["runtime-limit"].as_integer().unwrap_or(1000000) as u64,
            stderr: table["stderr"].as_str().unwrap_or("/dev/stderr").to_string(),
            stdin: table["stdin"].as_str().unwrap_or("/dev/stdin").to_string(),
            stdout: table["stdout"].as_str().unwrap_or("/dev/stdout").to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_correct() {
        let config_str = "
            cwd = 'path' # The working directory of spawned process
            large-stack = true # Enable large stack
            max-process = 11 # Process limit
            memory-limit = 1 # Memory usage limit in bytes
            output = 'path/to/output' # The path to output sandbox result
            output-size-limit = 1 # Output size limit in bytes
            runtime-limit = 1 # Runtime limit in millisecond
            stderr = 'path/to/stderr' # The stderr path of the spawned process
            stdin = 'path/to/stdin' # The stdin path of the spawned process
            stdout = 'path/to/stdout' # The stdout path of the spawned process";

        let config = Env::new(&config_str).unwrap();
        assert_eq!(config.cwd, "path");
        assert_eq!(config.large_stack, true);
        assert_eq!(config.max_process, 11);
        assert_eq!(config.output_file, "path/to/output");
        assert_eq!(config.output_size_limit, 1);
        assert_eq!(config.runtime_limit, 1);
        assert_eq!(config.stderr, "path/to/stderr");
        assert_eq!(config.stdin, "path/to/stdin");
        assert_eq!(config.stdout, "path/to/stdout");
    }

}