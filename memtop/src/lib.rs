use std::collections::HashMap;

#[derive(Debug)]
pub struct MemInfo {
    pub total_kb: u64,
    pub available_kb: u64,
}

pub const MEMINFO_FILE: &str = "/proc/meminfo";

impl MemInfo {
    pub fn used_percent(&self) -> f64 {
        if self.total_kb == 0 {
            0_f64
        } else {
            (self.total_kb as f64 - self.available_kb as f64) * 100.0_f64 / self.total_kb as f64
        }
    }
}

fn parse_meminfo_key(raw_parsed_meminfo: &HashMap<&str, &str>, key: &str) -> Option<u64> {
    if let Some(value_string_unit) = raw_parsed_meminfo.get(key) {
        if let Some((value_string, unit)) = value_string_unit.trim().split_once(" ")
            && unit == "kB"
        {
            return value_string.parse::<u64>().ok();
        }
    }
    return None;
}

pub fn parse_meminfo(text: &str) -> Option<MemInfo> {
    let raw_parsed_meminfo: HashMap<&str, &str> =
        text.lines().filter_map(|l| l.split_once(":")).collect();
    let mut result = MemInfo {
        total_kb: 0,
        available_kb: 0,
    };
    if let Some(mem_total_value) = parse_meminfo_key(&raw_parsed_meminfo, "MemTotal")
        && mem_total_value > 0
    {
        result.total_kb = mem_total_value;
    } else {
        return None;
    }
    if let Some(mem_avail_value) = parse_meminfo_key(&raw_parsed_meminfo, "MemAvailable") {
        result.available_kb = mem_avail_value;
    } else {
        return None;
    }

    return Some(result);
}

