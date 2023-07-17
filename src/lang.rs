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
            0 => {
                Ok(Self::C)
            }
            1 => {
                Ok(Self::CPP)
            }
            2 => {
                Ok(Self::PYTHON)
            }
            _ => {
                Err("Language id not found")
            }
        }
    }
}

impl Lang {
    pub fn get_compile_argv(self) -> Vec<CString> {
        match self {
            Self::C => {
                vec![CString::new("gcc").unwrap(), CString::new("-DONLINE_JUDGE").unwrap(), CString::new("-O2").unwrap(), CString::new("-w").unwrap()
                     , CString::new("-fmax-errors=3").unwrap(), CString::new("-std=c11").unwrap(), CString::new("main.c").unwrap(), CString::new("-lm").unwrap(), CString::new("-o").unwrap()
                     , CString::new("main").unwrap()]
            }
            Self::CPP => {
                vec![CString::new("g++").unwrap(), CString::new("-DONLINE_JUDGE").unwrap(), CString::new("-O2").unwrap()
                     , CString::new("-w").unwrap(), CString::new("-fmax-errors=3").unwrap(), CString::new("-std=c++17").unwrap(), CString::new("main.cpp").unwrap()
                     , CString::new("-lm").unwrap(), CString::new("-o").unwrap(), CString::new("main").unwrap()]
            }
            Self::PYTHON => {
                vec![]
            }
        }
    }

    pub fn get_execute_argv(self) -> Vec<CString> {
        match self {
            Self::C => {
                vec![CString::new("./main").unwrap()]
            }
            Self::CPP => {
                vec![CString::new("./main").unwrap()]
            }
            Self::PYTHON => {
                vec![CString::new("/usr/bin/python3").unwrap(), CString::new("main.py").unwrap()]
            }
        }
    }
}


