# Note: before using this script, ensure that you have the 'kubectl' command installed and configured to connect to your Kubernetes cluster.
# Also, ensure that you have the 'kustomize' command installed.
# Finally, ensure that you have the 'helm' command installed and the 'traefik' helm chart installed in your Kubernetes cluster:
# See instructions in ./deploy/k8s/overlays/development/helm/traefik-values.yaml
# One last thing, create the 'rusty-containers' namespace: kubectl create namespace rusty-containers.
kubectl kustomize ./deploy/k8s/overlays/development | kubectl create -f -
