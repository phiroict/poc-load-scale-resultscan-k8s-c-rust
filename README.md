# PoC k8s saturation 

Creates a k8s cluster and deploy a ha deployment and then clobbers it with a jmeter session to see how scaling works 

The result of the jmeter action is parsed by a c app and a rust app for comparisment. 

# Stack 

- minikube
- kubectl
- jmeter 
- gcc 
- rustup
- python 

OS: tested on macos Metal platform (ARM) and linux (arch) 

# Usage 

Use the minikube - kubectl to create the deployment. 
Note you need to add a line to the /etc/hosts file 

First get the ip address from minikube 

```bash
minikube ip
```
Then add this line to the hosts file 

```text
<you minikube ip address here> mydeploy-ex6.apps-crc.testing
```
for instance 
```text
192.168.39.66 mydeploy-ex6.apps-crc.testing
```

```bash
make start_minikube # AMD linux 
make start_minikube_arm # Mac Metal or ARM Linux
make deploy_k8s_components
make compile_c_scanner
make loadtest
```

## Mac & Windows 

On the mac and windows where you need an abstraction for the docker runtime you need to create a tunnel first before the 
ingress works. 
The tunnel needs to stay open and you need to be sudo users as it needs to set up routing on the mac/windows. 
```bash
minikube tunnel 
```

# Security 

The pods are set up with the minimum security surface and using an image not needing any capabilities.  

# Load tests 
## Run tests
With the jmeter application we can test the scaling and saturation. Now note that you can crash your cluster with this, proving the 
use of such a test. 
The test is pretty simple, just get the home page of the nginx installation.
There is another jmeter script for arm as the original one does not work 

## Parse tests 
First you need to run the jmeter task to test, after that you can summate the results with the tools. 

There are three parsing applictions for the jmeter results. These do the same thing and are more a comparing. 
- A c application `scanner.c`
- A rust app that has been converted from the scanner c code. `[scanner-rust](scanner-rust)`
- A rust application written as pure rust `[scanner-rust-native](scanner-rust-native)`
### Compile c 

In the `scanner.c` file you need to set the path the result file. (The default will not work for you)
```
const char* source = "/Users/phiro/Dropbox/Projects/kubernetes/CKAD/training/6/scratch/testresults.jtl";
```


You can compile the tool with
```bash
gcc scanner.c -o scanner 
```

### Rust apps

#### Scanner-rust
This is converted from the `scanner.c` application into rust and is an experiment, it does not use much of Rust's facilities
and just wraps the code in unsafe c-shim code. 

#### Scanner-rust-native. 
Change the path to the result set by altering this line 

```rust
let path = "/Users/phiro/Dropbox/Projects/kubernetes/CKAD/training/6/scratch/testresults.jtl";
```


You can build this code by 
```bash
make build
```

Then run the set by 

```bash
make run 
```
