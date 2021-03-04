# Introduction
In simple terms, RxSSH can be used to execute commands on a remote host via SSH. One advantage over the standard SSH client is that RxSSH let's you define a list of commands and hostnames, and then execute multiple commands on multiple hostnames. This comes in handy if, for instance, you are a system or network admin and have to execute multiple tasks (e.g. install software or change settings) on multiple nodes at the same time.

# Future functionality
* Start RxSSH as a daemon (either in client or server mode) and accept the commands via websocket
* UI visualization of nodes in the network and a possibility to interact with them, remotely execute commands from the UI (using something like: (https://bubbleplait.com)).

# Code
Check the code [here](https://github.com/aolchawa/rxssh)