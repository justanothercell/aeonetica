#!/usr/bin/python3

import subprocess
import sys
import os

abspath = os.path.abspath(__file__)
dname = os.path.dirname(abspath)

BOLD = '\033[1m'
ENDC = '\033[0m'
BLUE = '\033[94m'
GREEN = '\033[92m'

mods = []

def fetch_mods(ron_file):
    global mods
    print(f'{BLUE}{BOLD}=>> FETCHING MODS: {ENDC}')
    
    with open(ron_file, 'r') as file:
        for line in file:
            line = line.strip()
            if not line.startswith('"'):
                continue
            
            mod = line[1:line.find(':')]
            mods.append(mod)
            print(f'{GREEN} -> found `{mod}`')

if __name__ == '__main__':
    fetch_mods(f'{dname}/mods/mods.ron')
    os.chdir(f'{dname}/../mods')
    for mod in mods:
        print(f'{BLUE}{BOLD}=>> COMPILING MOD {mod}:{ENDC}')
        subprocess.call([sys.executable, 'build.py', '-w', mod, '-d', '../server/mods'])
    os.chdir(dname)
    print(f'{BLUE}{BOLD}=>> COMPILING SERVER: {ENDC}')
    subprocess.call(['cargo', 'run' if len(sys.argv) > 1 and sys.argv[1] in ['-r', '--run'] else 'build'])
