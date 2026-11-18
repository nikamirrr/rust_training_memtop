use std::collections::HashMap;

#[derive(Debug)]
pub struct MemInfo {
    pub total_kb: u64,
    pub available_kb: u64,
}

pub const MEMINFO_FILE: &str = "/proc/meminfo";

impl MemInfo {
    fn used_percent(&self) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_used() {
        let test_mem = MemInfo {
            total_kb: 10,
            available_kb: 10,
        };
        assert_eq!(test_mem.used_percent(), 0_f64);
    }

    #[test]
    fn zero_total() {
        let test_mem = MemInfo {
            total_kb: 0,
            available_kb: 10,
        };
        assert_eq!(test_mem.used_percent(), 0_f64);
    }

    #[test]
    fn parse_empty() {
        assert!(parse_meminfo("").is_none());
    }

    #[test]
    fn parse_garbage() {
        assert!(parse_meminfo("wfdmweprjweprjp23j4\nsdfsdf23\n\n4").is_none());
    }

    #[test]
    fn parse_mem_total_zero() {
        assert!(parse_meminfo("MemTotal:       0 kB\nMemFree:         8055840 kB\nMemAvailable:   13495428 kB\nBuffers:          296936 kB\n").is_none());
    }

    #[test]
    fn parse_no_both_total_avail() {
        assert!(
            parse_meminfo("MemFree:         8055840 kB\nBuffers:          296936 kB\n").is_none()
        );
    }

    #[test]
    fn parse_no_avail() {
        assert!(
            parse_meminfo(
                "MemTotal:       0 kB\nMemFree:         8055840 kB\nBuffers:          296936 kB\n"
            )
            .is_none()
        );
    }

    #[test]
    fn parse_no_total() {
        assert!(parse_meminfo("MemAvailable:   13495428 kB\nMemFree:         8055840 kB\nBuffers:          296936 kB\n").is_none());
    }

    #[test]
    fn parse_mem_normal() {
        let test_mem = parse_meminfo("MemTotal:       16355140 kB\nMemFree:         8055840 kB\nMemAvailable:   13495428 kB\nBuffers:          296936 kB\n").expect("parse_meminfo returned None for valid input");
        assert_eq!(test_mem.total_kb, 16355140);
        assert_eq!(test_mem.available_kb, 13495428);
    }
}
