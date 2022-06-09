# flatfetch
flatfetch is sort of like a __wget__ rust rewrite that I made.

It is kind of buggy so If you would like to make any changes I will accept them!

Code written by Matteo Rosato

### Installation On Linux Or Mac

first command:

chmod +x linuxmacos-installer.sh

second command:

./linuxmacos-installer.sh

and you are pretty much done!

To check if you have it run 

flatfetch --help

and this should show up:
FlatFetch 0.1.0
MRRCode979 <matteorosato979@gmail.com>
flatfetch is a wget clone (sort of) written in Rust that downloads files. I have a plan to make a
package manager in Rust in the future!

License GPLv3+: GNU GPL version 3 or later
flatfetch  Copyright (C) 2022  Matteo Rosato
This program comes with ABSOLUTELY NO WARRANTY; This is free software, and you are welcome to
redistribute it under certain conditions

USAGE:
    flatfetch <URL>

ARGS:
    <URL>    Url = [URL for the file (must begin in http:// https:// or else it wont work) ]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information


There is currently no installer for windows.
     
## Uninstall flatfetch:
command 1: 

chmod +x uninstaller.sh
        
command 2:
        
./uninstaller.sh

## TODO:

1. Add multithreading
2. Fix buggy code 
3. Download files in chunks instead of all at once.
4. add a Script to install flatfetch on windows
        
That is pretty much it! Hope you enjoy using flatfetch!
