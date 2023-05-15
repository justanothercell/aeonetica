#!/usr/bin/python3

import subprocess
import sys
import os

abspath = os.path.abspath(__file__)
dname = os.path.dirname(abspath)

mods = ['world', 'player']

BOLD = '\033[1m'
ENDC = '\033[0m'
BLUE = '\033[94m'

if __name__ == '__main__':
    os.chdir(f'{dname}/../mods')
    for mod in mods:
        print(f'{BLUE}{BOLD}=>> COMPILING MOD {mod}:{ENDC}')
        subprocess.call([sys.executable, 'build.py', '-w', mod, '-d', '../server/mods'])
    os.chdir(dname)
    print(f'{BLUE}{BOLD}=>> COMPILING SERVER: {ENDC}')
    subprocess.call(['cargo', 'run' if len(sys.argv) > 1 and sys.argv[1] in ['-r', '--run'] else 'build'])
