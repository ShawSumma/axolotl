# package Management

## Versioning
Versioning will be done using git commit hashes, each git repo used for a package will be forked to ensure that in the case of a git history being wiped, dependant packages don't suffer.

## Requirements
The package manager must be able to not only just in time compile code, maintain atomic versioning, and execute quickly, but it must also be able to display required commands used by the scripts it manages, for example if a program uses echo, curl, htop, the requirement for these commands existences must be prominently displayed, supposing that they aren't found. 