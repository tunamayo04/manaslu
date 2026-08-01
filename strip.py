#!/usr/bin/env python3
import sys

for line in sys.stdin:
    line = line.rstrip('\n')
    # Find "SP:" and keep everything up to and including the 2 hex digits after it
    sp_pos = line.find("SP:")
    if sp_pos != -1:
        end_pos = sp_pos + 5  # "SP:XX" is 5 characters total
        print(line[:end_pos])
    else:
        print(line)
