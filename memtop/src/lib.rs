use std::collections::HashMap;
use std::fmt;

#[derive(Default, Debug)]
pub struct MemInfo {
    pub total_kb: u64,
    pub available_kb: u64,
    pub free_kb: u64,
    pub buffers_kb: u64,
    pub cached_kb: u64,
    pub swaptotal_kb: u64,
    pub swapfree_kb: u64,
}

pub const MEMINFO_FILE: &str = "meminfo";

impl MemInfo {
    pub fn used_percent(&self) -> f64 {
        if self.total_kb == 0 || self.total_kb < self.available_kb {
            0_f64
        } else {
            (self.total_kb - self.available_kb) as f64 * 100.0_f64 / self.total_kb as f64
        }
    }
}

impl fmt::Display for MemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "system total_kb={} available_kb={} used_percent={:.2}",
            self.total_kb,
            self.available_kb,
            self.used_percent()
        )
    }
}

fn parse_meminfo_key(raw_parsed_meminfo: &HashMap<&str, &str>, key: &str) -> Option<u64> {
    if let Some(value_string_unit) = raw_parsed_meminfo.get(key)
        && let Some((value_string, unit)) = value_string_unit.trim().split_once(" ")
        && unit == "kB"
    {
        value_string.parse::<u64>().ok()
    } else {
        None
    }
}

pub fn parse_meminfo(text: &str) -> Option<MemInfo> {
    let raw_parsed_meminfo: HashMap<&str, &str> =
        text.lines().filter_map(|l| l.split_once(":")).collect();
    let mut result = MemInfo {
        total_kb: parse_meminfo_key(&raw_parsed_meminfo, "MemTotal")?,
        ..Default::default()
    };

    if result.total_kb == 0 {
        return None;
    }
    result.available_kb = parse_meminfo_key(&raw_parsed_meminfo, "MemAvailable")?;
    if result.available_kb > result.total_kb {
        return None;
    }

    result.free_kb = parse_meminfo_key(&raw_parsed_meminfo, "MemFree")?;
    result.buffers_kb = parse_meminfo_key(&raw_parsed_meminfo, "Buffers")?;
    result.cached_kb = parse_meminfo_key(&raw_parsed_meminfo, "Cached")?;
    result.swaptotal_kb = parse_meminfo_key(&raw_parsed_meminfo, "SwapTotal")?;
    result.swapfree_kb = parse_meminfo_key(&raw_parsed_meminfo, "SwapFree")?;

    Some(result)
}
