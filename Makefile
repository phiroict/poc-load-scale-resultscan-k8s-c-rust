loadtest:
	jmeter -n -t Kube\ clobberer.jmx -l testresults.jtl
loadtest_mac:
	jmeter -n -t Kube\ clobberer_mac_m1.jmx -l testresults.jtl
start_minikube:
	minikube start --driver kvm --cpus 8 --memory 32000
	minikube addons enable metrics-server
	minikube addons enable ingress
start_minikube_arm:
	minikube start --driver docker --cpus 20 --memory 32000
	minikube addons enable metrics-server
	minikube addons enable ingress
compile_c_scanner:
	gcc scanner.c -o scanner
	cd scanner-rust-native && make build
deploy_k8s_components:
	kubectl apply -f os-pod.yaml
	kubectl apply -f os-deployment.yaml
	kubectl apply -f os-deploy-service.yaml
	kubectl apply -f k8s-ingress-mydeploy.yaml
	kubectl apply -f os-mydeploy-hpa.yaml


