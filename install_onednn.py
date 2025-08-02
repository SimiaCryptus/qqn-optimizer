#!/usr/bin/env python3
"""
OneDNN Installation Script for Ubuntu/Debian systems

This script installs Intel's OneDNN library which is required for the OneDNN feature
of the qqn-optimizer project.
"""

import subprocess
import sys
import os

def run_command(cmd, check=True):
    """Run a shell command and return its result"""
    print(f"Running: {cmd}")
    try:
        result = subprocess.run(cmd, shell=True, check=check, capture_output=True, text=True)
        if result.stdout:
            print(result.stdout)
        return result
    except subprocess.CalledProcessError as e:
        print(f"Error running command: {e}")
        print(f"Stderr: {e.stderr}")
        if check:
            sys.exit(1)
        return e

def install_onednn_ubuntu():
    """Install OneDNN on Ubuntu/Debian systems"""
    print("Installing OneDNN for Ubuntu/Debian...")
    
    # Update package list
    run_command("sudo apt-get update")
    
    # Install required dependencies
    run_command("sudo apt-get install -y build-essential cmake git libc6-dev build-essential clang libclang-dev")

    # Install Intel oneAPI (which includes OneDNN)
    commands = [
        "wget -O- https://apt.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS.PUB | gpg --dearmor | sudo tee /usr/share/keyrings/oneapi-archive-keyring.gpg > /dev/null",
        "echo 'deb [signed-by=/usr/share/keyrings/oneapi-archive-keyring.gpg] https://apt.repos.intel.com/oneapi all main' | sudo tee /etc/apt/sources.list.d/oneAPI.list",
        "sudo apt-get update",
        "sudo apt-get install -y intel-oneapi-dnnl-devel"
    ]
    
    for cmd in commands:
        run_command(cmd)
    
    # Set up environment variables
    env_setup = """
# Add these lines to your ~/.bashrc or ~/.zshrc
export DNNL_ROOT=/opt/intel/oneapi/dnnl/latest
export PKG_CONFIG_PATH=$DNNL_ROOT/lib/pkgconfig:$PKG_CONFIG_PATH
export LD_LIBRARY_PATH=$DNNL_ROOT/lib:$LD_LIBRARY_PATH
"""
    
    print("\n" + "="*60)
    print("OneDNN installation completed!")
    print("Add the following to your shell configuration:")
    print(env_setup)
    print("="*60)

def install_onednn_source():
    """Install OneDNN from source"""
    print("Installing OneDNN from source...")
    
    # Clone the repository
    run_command("git clone https://github.com/oneapi-src/oneDNN.git")
    run_command("cd oneDNN")
    
    # Build and install
    commands = [
        "mkdir build",
        "cd build",
        "cmake .. -DCMAKE_INSTALL_PREFIX=/usr/local",
        "make -j$(nproc)",
        "sudo make install"
    ]
    
    for cmd in commands:
        run_command(f"cd oneDNN && {cmd}")
    
    print("OneDNN source installation completed!")

def main():
    """Main installation function"""
    print("OneDNN Installation Script for qqn-optimizer")
    print("=" * 50)
    
    if len(sys.argv) > 1 and sys.argv[1] == "--source":
        install_onednn_source()
    else:
        # Try Ubuntu/Debian installation first
        try:
            install_onednn_ubuntu()
        except:
            print("\nUbuntu/Debian installation failed. Trying source installation...")
            install_onednn_source()
    
    print("\nTo test the installation, run:")
    print("source /opt/intel/oneapi/setvars.sh")
    print("cargo build --features onednn")
    print("cargo test --features onednn")

if __name__ == "__main__":
    main()