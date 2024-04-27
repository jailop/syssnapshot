# syssnapshot - Report cpu, memory, disk, and network usage

## Installation

In order to install this progrom in your computer, you need to install the Rust
build system.

    $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    
Clone this repository in your local disk:

    $ git clone git@github.com:jailop/syssnapshot.git
    
Build and install the binary:

    $ cd syssnapshot
    $ make
    $ sudo make install
    
At this point you'll be able to run this program:

    $ syssnapshot
