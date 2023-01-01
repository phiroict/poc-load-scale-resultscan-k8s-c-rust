loadtest:
	jmeter -n -t Kube\ clobberer.jmx -l testresults.jtl
start_minikube:
	minikube start --driver kvm --cpus 8 --memory 32000
compile_c_scanner:
	gcc scanner.c -o scanner