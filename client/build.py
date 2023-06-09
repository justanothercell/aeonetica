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
    build_cmd = ['cargo', 'run' if '-r' in sys.argv or '--run' in sys.argv else 'build']
    if '--release' in sys.argv:
        build_cmd += ['--release']
    
    subprocess.call(build_cmd)
