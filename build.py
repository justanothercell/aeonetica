#!/usr/bin/python3

import subprocess
import platform
import zipfile
import sys
import os

import server.build as server

osname = platform.system().lower()
binary_ext = '.exe' if osname == 'windows' else ''
target_platform = f'{platform.machine()}-{osname}'

abspath = os.path.abspath(__file__)
dname = os.path.dirname(abspath)

BOLD = '\033[1m'
ENDC = '\033[0m'
BLUE = '\033[94m'
GREEN = '\033[92m'

if __name__ == '__main__':
    os.chdir(dname)
    mode = ['--release'] if '--release' in sys.argv[1:] else []

    print(f'Selected build mode: {BOLD}{mode}{ENDC}')

    processes = [
        ('server', subprocess.Popen(['cargo', 'build', *mode],
                                    cwd=dname+'/server',
                                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)),
        ('client', subprocess.Popen(['cargo', 'build', *mode],
                                    cwd=dname+'/client',
                                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)),
    ]
    
    for mod in server.fetch_mods('server/mods/mods.ron'):
        process = subprocess.Popen([sys.executable, 'build.py', '-w', mod, '-d', '../server/mods', *mode],
                                   cwd=dname+'/mods', 
                                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        processes.append((f'mods/{mod}', process))
    
    total = len(processes)
    
    print(f'{BOLD}{BLUE}=>> COMPILING EVERYTHING:{ENDC}')
    print(f'Queued {total} builds: {[x[0] for x in processes]}')

    def finished(n, p):
        status = p.poll()
        if status is None:
            return False
        print(f'completed build {BOLD}{n}{ENDC} with exit status: {status} ({total - len(processes) + 1} of {total})')
        return True

    while len(processes) > 0:
        processes[:] = [x for x in processes if not finished(*x)]
        
    if '-p' in sys.argv[1:] or '--package' in sys.argv[1:]:
        mode = 'release' if '--release' in mode else 'debug'
        
        print(f'{BOLD}{BLUE}=>> PACKAGING FOR PLATFORM {target_platform}{ENDC}')
        
        # package client
        client_bin = 'client' + binary_ext
        client_package = f'client-{target_platform}-{mode}.zip'
        print(f'{GREEN} -> generating `{client_package}`{ENDC}')
        
        with zipfile.ZipFile(client_package, 'w', zipfile.ZIP_DEFLATED) as zipf:
            zipf.write(f'client/target/{mode}/{client_bin}', client_bin)
        
        # package server and mods
        server_bin = 'server' + binary_ext
        server_package = f'server-{target_platform}-{mode}.zip'
        print(f'{GREEN} -> generating `{server_package}`{ENDC}')
        
        with zipfile.ZipFile(server_package, 'w', zipfile.ZIP_DEFLATED) as zipf:
            zipf.write(f'server/target/{mode}/{server_bin}', server_bin)
            
            for file in os.listdir(os.fsencode('server/mods')):
                filename = 'mods/' + os.fsdecode(file)
                if filename.endswith('.zip') or filename.endswith('.ron'):
                    zipf.write('server/' + filename, filename)
        
    print(f'{BOLD}done.{ENDC}')