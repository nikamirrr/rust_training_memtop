#[cfg(test)]
mod tests {
    use memtop::*;

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
