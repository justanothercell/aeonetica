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

def fetch_mods(ron_file):
    mods = []
    print(f'{BLUE}{BOLD}=>> FETCHING MODS: {ENDC}')
    
    with open(ron_file, 'r') as file:
        for line in file:
            line = line.strip()
            if not line.startswith('"'):
                continue
            
            mod = line[1:line.find(':')]
            mods.append(mod)
            print(f'{GREEN} -> found `{mod}` {ENDC}')
    return mods

if __name__ == '__main__':
    os.chdir(f'{dname}/../mods')
    
    mode = ['--release'] if '--release' in sys.argv[1:] else []
    
    server_mode = ['--release'] if '--sr' in sys.argv[1:] else []

    envs = {**os.environ.copy(), 'RUSTFLAGS': '-Awarnings'}
    
    mod_override = [i for i, arg in enumerate(sys.argv) if arg == '-m' or arg == '--mods']
    mods = fetch_mods(f'{dname}/mods/mods.ron') if len(mod_override) == 0 else [] if len(sys.argv) - mod_override[0] == 1 else [sys.argv[mod_override[0] + 1]]

    print(f'{BOLD}{BLUE}=>> COMPILING SELECTED MODS:{ENDC}')

    processes = []
    for mod in mods:
        print(f'{BLUE}{BOLD}=>> COMPILING MOD {mod}{ENDC}')
        process = subprocess.Popen([sys.executable, 'build.py', '-w', mod, '-d', '../server/mods', *mode], env=envs)
        processes.append((f'mods/{mod}', process))
    
    total = len(processes)
    print(f'Queued {total} builds: {[x[0] for x in processes]}')

    finished_msgs = []

    def finished(n, p):
        status = p.poll()
        if status is None:
            return False
        print(f'completed build {BOLD}{n}{ENDC} with exit status: {status} ({total - len(processes) + 1} of {total})')
        finished_msgs.append(f'{BOLD}{n}{ENDC} with exit status {status}')
        return True
        
    while len(processes) > 0:
        processes[:] = [x for x in processes if not finished(*x)]
    
    print(f'{BOLD} ALL MODS BUILT: {ENDC}')
    for msg in finished_msgs:
        print(f'    {msg}')
    
    os.chdir(dname)
    print(f'{BLUE}{BOLD}=>> COMPILING SERVER: {ENDC}')
    
    subprocess.call(['cargo', 'run' if '-r' in sys.argv or '--run' in sys.argv else 'build', *server_mode], env=envs)
