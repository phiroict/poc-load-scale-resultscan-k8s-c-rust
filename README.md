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
make start_minikube
make deploy_k8s_components
make compile_c_scanner
make loadtest
```
