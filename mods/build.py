#!/usr/bin/python

from os import *
import sys
import shutil
import zipfile
import contextlib
import subprocess

@contextlib.contextmanager
def pushd(new_dir):
    previous_dir = getcwd()
    chdir(new_dir)
    try:
        yield
    finally:
        chdir(previous_dir)

mod_name = None
_a, _b, _c, _d = subprocess.check_output(['rustc', '-vV']).decode('utf-8').split('\n')[4][6:].split('-')
mod_target = _a + '-' + _c.replace('linux', 'unix')
build_mode = 'debug'
deploy_path = ''
output_file = ''
help_text = f"""Usage: ./build.py [options]
Options:
    -h, --help              | displays this help text
    -r, --release           | build in release mode (default: {build_mode})
    -d, --deploy <dir>      | deploy the mod to <dir>
    -o, --output <file>     | set the output file
    -w, --working-dir <dir> | set the working directory (default: script directory)
    -z, --only-zip          | do not recompile, only re-zip
        --clean             | clean build files"""
 
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

def build(feature: str):
    # run cargo build command
    build_cmd = f'cargo rustc --features="{feature}" --crate-type=dylib'
    if build_mode == 'release':
        build_cmd += ' --release'
    res = system(build_cmd)
    if res != 0:
        print(f'build failed with error code {res}')
        exit(1)

def zippify(prefix: str, feature: str):
    # cd into the build directory
    with pushd(prefix):
        # check if compilation was successful
        output_file = get_output_file(feature)
        if not path.exists(output_file):
            raise Exception(f'Error retreiving \'{output_file}\': No such file or directory')

        feature_name = feature
        if feature == 'client':
            feature_name += f'-{mod_target}'

        out_dir = f'out/{feature_name}'
        makedirs(out_dir, exist_ok=True)

        # package file
        target_bin = f'{mod_name}_{feature}.{get_mod_file_ext()}'
        shutil.copy(output_file, f'{out_dir}/{target_bin}')

        archive = path.abspath(f'{mod_name}_{feature_name}.zip')
        with pushd(out_dir):
            with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED) as zipf:
                zipf.write(target_bin)
        return path.basename(archive)

def deploy(archive: str):
    name = path.basename(archive)
    if not path.exists(deploy_path):
        raise Exception(f'Error deploying \'{mod_name}\' to \'{deploy_path}\': No such file or directory')
    print(f"Deploying {archive} to {deploy_path}...")
    shutil.copy(archive, deploy_path)

def clean():
    system('cargo clean')
    
def compile_mod(prefix: str, archive: str):    
    print(f'Building \'{mod_name}\' in {build_mode} mode...')

    if build_mode == 'release':
        build('client')
        zc = zippify(prefix, 'client')
        build('server')
        zs = zippify(prefix, 'server')
        archives = [zc, zs]
    else:
        build('client server')
        archives = [zippify(prefix, 'server'), zippify(prefix, 'client')]

    # build the final zip file
    with pushd(prefix):
        with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED) as zipf:
            for a in archives:
                zipf.write(a)

if __name__ == '__main__':  
    working_dir = path.dirname(path.abspath(__file__))
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
            elif arg in ['--working-dir', '-w']:
                working_dir = next(it)
            elif arg == '--clean':
                clean()
                exit()
            else:
                raise Exception(f'Unknown argument {arg}')

    mod_name = path.basename(working_dir)
    prefix = f'target/{build_mode}'
    archive = f'{mod_name}.zip'
    with pushd(working_dir):
        compile_mod(prefix, archive)
        
    if len(output_file) > 0:
        shutil.copy(f'{working_dir}/{prefix}/{archive}', output_file)     
    
    if len(deploy_path) > 0:
        deploy(f'{working_dir}/{prefix}/{archive}')
