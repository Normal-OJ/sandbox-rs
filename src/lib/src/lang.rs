use std::ffi::CString;

#[derive(Copy, Clone)]
pub enum Lang {
    C,
    CPP,
    PYTHON,
}

impl TryFrom<u64> for Lang {
    type Error = &'static str;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::C),
            1 => Ok(Self::CPP),
            2 => Ok(Self::PYTHON),
            _ => Err("Language id not found"),
        }
    }
}

fn rust_string_vec_to_c_string_vec(vec: Vec<&'static str>) -> Vec<CString> {
    vec.iter().map(|e| CString::new(*e).unwrap()).collect()
}

impl Lang {
    pub fn get_compile_argv(self) -> Vec<CString> {
        match self {
            Self::C => rust_string_vec_to_c_string_vec(vec![
                "gcc",
                "-DONLINE_JUDGE",
                "-O2",
                "-w",
                "-fmax-errors=3",
                "-std=c11",
                "main.c",
                "-lm",
                "-o",
                "main",
            ]),
            Self::CPP => rust_string_vec_to_c_string_vec(vec![
                "g++",
                "-DONLINE_JUDGE",
                "-O2",
                "-w",
                "-fmax-errors=3",
                "-std=c++17",
                "main.cpp",
                "-lm",
                "-o",
                "main",
            ]),
            Self::PYTHON => {
                vec![]
            }
        }
    }

    pub fn get_execute_argv(self) -> Vec<CString> {
        match self {
            Self::C => rust_string_vec_to_c_string_vec(vec!["./main"]),
            Self::CPP => rust_string_vec_to_c_string_vec(vec!["./main"]),
            Self::PYTHON => rust_string_vec_to_c_string_vec(vec!["/usr/bin/python3", "main.py"]),
        }
    }
}
