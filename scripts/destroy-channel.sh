cd deployments/environments/channel || exit

terraform init && terraform destroy -auto-approve

rm -rf "../../output/$TF_WORKSPACE"
