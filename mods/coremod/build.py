#!/usr/bin/python

from os import *
import sys
import shutil
import zipfile
import contextlib

@contextlib.contextmanager
def pushd(new_dir):
    previous_dir = getcwd()
    chdir(new_dir)
    try:
        yield
    finally:
        chdir(previous_dir)

mod_name = path.basename(path.dirname(path.abspath(__file__)))
build_mode = 'debug'
deploy_path = ''
output_file = ''
help_text = f"""Usage: ./build.py [options]
Options:
    -h, --help          | displays this help text
    -r, --release       | build in release mode (default: {build_mode})
    -d, --deploy <dir>  | deploy the mod to <dir>
    -o, --output <file> | set the output file
        --clean         | clean build files"""
 
def get_output_file(feature: str):
    if sys.platform == 'linux' or sys.platform == 'linux2':
        return f'lib{mod_name}.so'
    elif sys.platform == 'win32' or sys.platform == 'cygwin':
        return f'{mod_name}.dll' 
    else:
        raise Exception('Unsupported platform {sys.platform}')
    
def get_mod_file_ext():
    if sys.platform == 'linux' or sys.platform == 'linux2':
        return 'so'
    elif sys.platform == 'win32' or sys.platform == 'cygwin':
        return 'dll' 
    else:
        raise Exception('Unsupported platform {sys.platform}')

def build(prefix: str, feature: str): 
    # run cargo build command
    build_cmd = f'cargo build --features={feature}'
    if build_mode == 'release':
        build_cmd += ' --release'
    system(build_cmd)
    
    # cd into the build directory
    with pushd(prefix):
        # check if compilation was successful
        output_file = get_output_file(feature)
        if not path.exists(output_file):
            raise Exception(f'Error retreiving \'{output_file}\': No such file or directory')

        out_dir = f'out/{feature}'
        makedirs(out_dir, exist_ok=True)

        # package file
        target_bin = f'{mod_name}_{feature}.{get_mod_file_ext()}'
        shutil.copy(output_file, f'{out_dir}/{target_bin}')

        archive = path.abspath(f'{mod_name}_{feature}.zip')
        with pushd(out_dir):
            with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED) as zipf:
                zipf.write(target_bin)
        return path.basename(archive)

def deploy(archive):
    name = path.basename(archive)
    if not path.exists(deploy_path):
            raise Exception(f'Error deploying \'{mod_name}\' to \'{deploy_path}\': No such file or directory')
    print(f"Deploying {archive} to {deploy_path}...")
    shutil.copy(archive, deploy_path)

def clean():
    system('cargo clean')

if __name__ == '__main__':  
    if len(sys.argv) > 1:
        it = iter(sys.argv[1:])
        for arg in it:
            if arg in ['--help', '-h']:
                print(help_text)                
                exit()
            elif arg in ['--output', '-o']:
                output_file = next(it)
            elif arg in ['--release', '-r']:
                build_mode = 'release'
            elif arg in ['--deploy', '-d']:
                deploy_path = next(it)
            elif arg == '--clean':
                clean()
                exit()
            else:
                raise Exception(f'Unknown argument {arg}')
    
    prefix = f'target/{build_mode}'
    
    print(f'Building \'{mod_name}\' in {build_mode} mode...')
    archives = [build(prefix, 'client'), build(prefix, 'server')]

    # build the final zip file
    with pushd(prefix):
        archive = f'{mod_name}.zip'
        with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED) as zipf:
            for a in archives:
                zipf.write(a)
                
    if len(output_file) > 0:\
        shutil.copy(f'{prefix}/{archive}', output_file)     
    
    if len(deploy_path) > 0:
        deploy(f'{prefix}/{archive}')
