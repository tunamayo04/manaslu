#!/usr/bin/env python3
import re
import sys

ppu_pattern = re.compile(r"\s+PPU:\s*\d+,\s*\d+\s+")

for line in sys.stdin:
    line = line.rstrip("\n")
    line = ppu_pattern.sub(" ", line)
    print(line)