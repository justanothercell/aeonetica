#!/usr/bin/python3

import subprocess
import sys
import os

abspath = os.path.abspath(__file__)
dname = os.path.dirname(abspath)

BOLD = '\033[1m'
ENDC = '\033[0m'
BLUE = '\033[94m'

if __name__ == '__main__':
    os.chdir(dname)
    print(f'{BOLD}{BLUE}=>> COMPILING CLIENT:{ENDC}')
    subprocess.call(['cargo', 'run' if len(sys.argv) > 1 and sys.argv[1] in ['-r', '--run'] else 'build'])
