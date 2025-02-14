provider "kubernetes" {
  config_path    = "~/.kube/config"
  config_context = "rancher-desktop"
}

module "simple_webapp" {
  source = "../../modules/services/k8s-app"

  name           = "simple-webapp"
  image          = "library/nginx"
  replicas       = 2
  container_port = 5000
}
