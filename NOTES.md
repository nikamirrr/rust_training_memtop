Day 1:
A bad MemTotal line would be ignored with filter_map if there is no ":" separator in parse_meminfo in that line, as a result
the MemTotal key would not be found
If on the MemTotal line the value is not numeric with the " kB" suffix the parse_meminfo_key would return None or fail with .ok() call


