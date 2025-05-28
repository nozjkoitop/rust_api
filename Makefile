ARGS = $(filter-out $@,$(MAKECMDGOALS))
%:
	@:

# --------------------------------------------------------------------------------------------------

build:
	@docker compose build

start:
	@docker compose up -d

up:
	@docker compose up

down:
	@docker compose down

top:
	@docker compose top

reload: down start

frontend-install:
	@docker compose run --rm frontend npm install

install: frontend-install
