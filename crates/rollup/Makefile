PROJECT_ROOT := $(shell git rev-parse --show-toplevel)
DOCKER_COMPOSE_DIR := $(PROJECT_ROOT)/docker
DOCKER_COMPOSE_CFG := $(DOCKER_COMPOSE_DIR)/docker-compose.yml
CREDENTIALS_DIR := $(DOCKER_COMPOSE_DIR)/credentials
SOV_CLI_REL_PATH := $(PROJECT_ROOT)/target/debug/starter-cli-wallet

# get the jwt token from Celestia
get_auth = $(shell cat $(CREDENTIALS_DIR)/$(KEY_NAME).jwt)

# Docker compose commands
docker_compose := docker compose -f $(DOCKER_COMPOSE_CFG)
# start the service
compose_up := $(docker_compose) up --build --force-recreate -d
# stop the service
compose_down := $(docker_compose) down
# follow the logs
compose_logs := $(docker_compose) logs -f
# check if service is running 
# we need a bridge and validator so naive check is to have at least 2 services
compose_is_running = test 2 -le "$(shell $(docker_compose) ps --filter status=running --format json | jq 'length')"


# start the celestia network and generate a new config
start: start-network generate-configs

start-network: check-docker 
	@echo "Starting local celestia network setup"
	@if $(compose_is_running); then echo "network is already running"; else $(compose_up); fi
	
# stop the celestia network
stop: check-docker
	@echo "Stopping local celestia network setup"
	@$(compose_down)

wait-compose-ready:
	@echo "Waiting for services to finish setup"
	@$(compose_logs) | awk '/Provisioning finished./ {print;exit}' # exit when encounter this log entry

generate-configs: wait-compose-ready
	@echo "Generating rollup config"
	@echo "running script at $(DOCKER_COMPOSE_DIR)/generate_configs.sh"
	@cd $(DOCKER_COMPOSE_DIR) && ./generate_configs.sh
	@cp $(DOCKER_COMPOSE_DIR)/rollup_config_0.toml $(PROJECT_ROOT)/rollup_config.toml

# listen to celestia network logs
compose-logs: check-docker
	@echo "Showing celestia logs"
	@$(compose_logs)

# check if docker is installed
check-docker:
	@command -v docker > /dev/null 2>&1 || { echo "Docker is not installed"; exit 1; }
	@# TODO: check docker compose too

clean-db:
	rm -rf demo_data_0

build-sov-cli:
	cargo build --bin starter-cli-wallet

set-rpc-url: build-sov-cli
	$(SOV_CLI_REL_PATH) rpc set-url "http://127.0.0.1:12340"

import-keys: build-sov-cli
	$(SOV_CLI_REL_PATH) keys import --nickname DANGER__DO_NOT_USE_WITH_REAL_MONEY --path ../../test-data/keys/token_deployer_private_key.json


test-generate-create-token-tx: build-sov-cli
	$(SOV_CLI_REL_PATH) transactions import from-file bank --path ../../test-data/requests/create_token.json


test-create-token: set-rpc-url test-generate-create-token-tx import-keys
	$(SOV_CLI_REL_PATH) rpc submit-batch by-nickname DANGER__DO_NOT_USE_WITH_REAL_MONEY

test-bank-supply-of: 
	curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"bank_supplyOf","params":["sov1zdwj8thgev2u3yyrrlekmvtsz4av4tp3m7dm5mx5peejnesga27svq9m72"],"id":1}' http://127.0.0.1:12345
