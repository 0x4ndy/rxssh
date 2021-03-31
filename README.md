# Introduction
In simple terms, RxSSH can be used to execute commands on a remote host via SSH. One advantage over the standard SSH client is that RxSSH let's you define a list of commands and hostnames, and then execute multiple commands on multiple hostnames. This comes in handy if, for instance, you are a system or network admin and have to execute multiple tasks (e.g. install software or change settings) on multiple nodes at the same time.

# Overall Project Status
RxSSH is currently only able to execute a single command on a single host (see [Example Section](#Example)).

Note: RxSSH is at a very early stage, therefore, expect not optimized or hardcoded elements in the code.

# Future functionality
* Configuration of list of hosts and/or commands to be executed (e.g. in JSON format)
* Start RxSSH as a daemon (either in client or server mode) and accept the commands via websocket
* UI visualization of nodes in the network and a possibility to interact with them, remotely execute commands from the UI (e.g. using something like [BubblePlait](https://bubbleplait.com)).
* Others (see [Issues](https://github.com/4ndyc0d3s/rxssh/projects/1) or [Projects](https://github.com/4ndyc0d3s/rxssh/projects/1) for details).

# Build
* Clone it from this repo
* Execute `cargo build --release` 
* Find generated binary under `<project>/release/rxssh`

# Configuration
RxSSH makes use of ssh-agent. In order for it to work properly and connect without passwords, make sure that your ssh-keys are set properly within the network as well as ssh agent is started. You can start the ssh-agent executing the following commands:
```
eval `ssh-agent -s` >/dev/null
```
and
```
ssh-add ~/.ssh/<ssh-key> 2>/dev/null
```
It is recommended to and these two lines to .bashrc

# Usage Examples
```
rxssh -c "ls -ltra" -u username -h example.com
```

To see more options, type:
```
rxssh --help
```

# Code
Check the code [here](https://github.com/4ndyc0d3s/rxssh)
