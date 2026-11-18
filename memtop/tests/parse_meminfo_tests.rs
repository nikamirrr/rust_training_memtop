#[cfg(test)]
mod tests {
    use memtop::*;

    #[test]
    fn zero_used() {
        let mut test_mem = MemInfo::default();
        test_mem.total_kb = 10;
        test_mem.available_kb = 10;
        assert_eq!(test_mem.used_percent(), 0_f64);
    }

    #[test]
    fn zero_total() {
        let mut test_mem = MemInfo::default();
        test_mem.total_kb = 0;
        test_mem.available_kb = 10;
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
    fn parse_total_less_then_avail() {
        assert!(parse_meminfo("MemTotal:       1635 kB\nMemAvailable:   13495428 kB\nMemFree:         8055840 kB\nBuffers:          296936 kB\n").is_none());
    }

    #[test]
    fn parse_mem_full() {
        let raw_data = r#"
MemTotal:       16355140 kB
MemFree:         8271212 kB
MemAvailable:   13421680 kB
Buffers:          320728 kB
Cached:          4636552 kB
SwapCached:         1404 kB
Active:          3841092 kB
Inactive:        3234152 kB
Active(anon):    2141848 kB
Inactive(anon):     2916 kB
Active(file):    1699244 kB
Inactive(file):  3231236 kB
Unevictable:       27464 kB
Mlocked:           27464 kB
SwapTotal:       4003836 kB
SwapFree:        3992668 kB
Zswap:                 0 kB
Zswapped:              0 kB
Dirty:              8176 kB
Writeback:             0 kB
AnonPages:       2144636 kB
Mapped:           777176 kB
Shmem:             18036 kB
KReclaimable:     561796 kB
Slab:             785624 kB
SReclaimable:     561796 kB
SUnreclaim:       223828 kB
KernelStack:       17632 kB
PageTables:        47828 kB
SecPageTables:         0 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:    12181404 kB
Committed_AS:   10876720 kB
VmallocTotal:   13743895347199 kB
VmallocUsed:       69196 kB
VmallocChunk:          0 kB
Percpu:             8800 kB
HardwareCorrupted:     0 kB
AnonHugePages:         0 kB
ShmemHugePages:        0 kB
ShmemPmdMapped:        0 kB
FileHugePages:         0 kB
FilePmdMapped:         0 kB
Unaccepted:            0 kB
HugePages_Total:       0
HugePages_Free:        0
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:       2048 kB
Hugetlb:               0 kB
DirectMap4k:      392120 kB
DirectMap2M:    11122688 kB
DirectMap1G:     7340032 kB
"#;
        let test_mem =
            parse_meminfo(raw_data).expect("parse_meminfo returned None for valid input");
        assert_eq!(test_mem.total_kb, 16355140);
        assert_eq!(test_mem.available_kb, 13421680);
        assert_eq!(test_mem.free_kb, 8271212);
        assert_eq!(test_mem.buffers_kb, 320728);
        assert_eq!(test_mem.cached_kb, 4636552);
        assert_eq!(test_mem.swaptotal_kb, 4003836);
        assert_eq!(test_mem.swapfree_kb, 3992668);
    }
}
