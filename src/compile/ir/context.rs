use std::fmt;
pub struct Context {
    source_filename: String,
    target_triple: TargetTriple,
    data_layout: DataLayout,
}

impl Context {
    pub fn new(source_filename: String) -> Self {
        let target_triple = TargetTriple::new();
        let data_layout = DataLayout::new();
        Self {
            source_filename: source_filename,
            target_triple: target_triple,
            data_layout: data_layout,
        }
    }
    pub fn dump(&self) {
        println!("source_filename = \"{}\"", self.source_filename);
        println!("target_triple = \"{}\"", self.target_triple.string());
        println!("data_layout = \"{}\"", self.data_layout.string());
    }
}

struct TargetTriple {
    cpu: CPU,
    vendor: Vendor,
    os: OS,
    abi: ABI,
}

impl TargetTriple {
    fn new() -> Self {
        Self {
            cpu: CPU::X64,
            vendor: Vendor::PC,
            os: OS::Linux,
            abi: ABI::GNU,
        }
    }
    fn string(&self) -> String {
        format!("{}-{}-{}-{}", self.cpu, self.vendor, self.os, self.abi)
    }
}

struct DataLayout {}

impl DataLayout {
    fn new() -> Self {
        Self {}
    }
    fn string(&self) -> &str {
        "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
    }
}

enum CPU {
    X64,
}
impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::X64 => write!(f, "x86-64"),
        }
    }
}

enum Vendor {
    PC,
}
impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PC => write!(f, "pc"),
        }
    }
}
enum OS {
    Linux,
}
impl fmt::Display for OS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Linux => write!(f, "linux"),
        }
    }
}
enum ABI {
    GNU,
}
impl fmt::Display for ABI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::GNU => write!(f, "gnu"),
        }
    }
}
