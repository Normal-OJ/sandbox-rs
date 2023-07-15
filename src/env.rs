use std::error::Error;
use toml::{Table, Value};

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
    fn new(src_str: &str) -> Result<Self, Box<dyn Error>> {
        let table = src_str.parse::<Table>()?;

        Ok(Self {
            cwd: table.get("cwd").unwrap_or(&Value::from("./")).as_str().ok_or("cwd should be String type")?.to_string(),
            large_stack: table.get("large-stack").unwrap_or(&Value::from(false)).as_bool().ok_or("large-stack should be Boolean type")?,
            max_process: table.get("max-process").unwrap_or(&Value::from(1)).as_integer().ok_or("max-process should be Integer type")? as i32,
            memory_limit: table.get("memory-limit").unwrap_or(&Value::from(1000000)).as_integer().ok_or("memory-limit should be Integer type")? as u64,
            output_file: table.get("output").unwrap_or(&Value::from("./out")).as_str().ok_or("output should be String type")?.to_string(),
            output_size_limit: table.get("output-size-limit").unwrap_or(&Value::from(100000)).as_integer().ok_or("output-size-limit should be Integer type")? as u64,
            runtime_limit: table.get("runtime-limit").unwrap_or(&Value::from(1000000)).as_integer().ok_or("runtime-limit should be Integer type")? as u64,
            stderr: table.get("stderr").unwrap_or(&Value::from("/dev/stderr")).as_str().ok_or("stderr should be String type")?.to_string(),
            stdin: table.get("stdin").unwrap_or(&Value::from("/dev/stdin")).as_str().ok_or("stdin should be String type")?.to_string(),
            stdout: table.get("stdout").unwrap_or(&Value::from("/dev/stdout")).as_str().ok_or("stdout should be String type")?.to_string(),
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
        assert_eq!(config.memory_limit, 1);
        assert_eq!(config.stderr, "path/to/stderr");
        assert_eq!(config.stdin, "path/to/stdin");
        assert_eq!(config.stdout, "path/to/stdout");
    }

    #[test]
    fn test_parse_error_format() {
        let config_str = "Lemino Chen";
        assert!(Env::new(&config_str).is_err());
    }

    #[test]
    fn test_use_default() {
        let config_str = "
            cwd = 'path' # The working directory of spawned process
            max-process = 11 # Process limit
            memory-limit = 1 # Memory usage limit in bytes
            output = 'path/to/output' # The path to output sandbox result
            output-size-limit = 1 # Output size limit in bytes
            stdin = 'path/to/stdin' # The stdin path of the spawned process
            stdout = 'path/to/stdout' # The stdout path of the spawned process";
        let config = Env::new(&config_str).unwrap();

        assert_eq!(config.large_stack, false);
        assert_eq!(config.stderr, "/dev/stderr");
        assert_eq!(config.runtime_limit, 1000000);
    }

    #[test]
    fn test_parse_error_type() {
        let config_str = "
            cwd = 'path' # The working directory of spawned process
            max-process = 11 # Process limit
            memory-limit = 1.1 # Memory usage limit in bytes
            output = 'path/to/output' # The path to output sandbox result
            output-size-limit = 1 # Output size limit in bytes
            runtime-limit = 1 # Runtime limit in millisecond
            stdin = 'path/to/stdin' # The stdin path of the spawned process
            stdout = 'path/to/stdout' # The stdout path of the spawned process";
        assert!(Env::new(&config_str).is_err());
    }
}