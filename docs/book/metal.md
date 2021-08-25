# Running Axolotl on bare metal

## Shell ecosystem.
Owing to the fact that axolotl is a shell scripting language, it is nessisary to define a means of handling the execution of external programs such as echo, ls, top, cat, and any other commands that may be dependencies of any given project. 

In order to maintain the request functionality on bare metal, a few functions must be marked to serve the purpose of the below listed unix sys calls.

1. [fork](https://man7.org/linux/man-pages/man2/fork.2.html)
2. [exec](https://man7.org/linux/man-pages/man3/exec.3.html)
3. [mmap](https://man7.org/linux/man-pages/man2/mmap.2.html)
4. [shm_open](https://man7.org/linux/man-pages/man3/shm_open.3.html)

Owing to the higher level nature of axolotol, the arguments given to these marked functions will look slightly different. however the structure they will take is to be determined. it may also be the case that these functions will be mapped in toolchain declaration file.