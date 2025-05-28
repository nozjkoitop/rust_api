ARGS = $(filter-out $@,$(MAKECMDGOALS))
%:
	@:

.PHONY: build start up down down-vol top reload frontend-install install rebuild
# --------------------------------------------------------------------------------------------------

build:
	@docker compose build

start:
	@docker compose up -d

up:
	@docker compose up

down:
	@docker compose down

down-vol:
	@docker compose down -v

top:
	@docker compose top

reload: down start

frontend-install:
	@docker compose run --rm frontend npm install

install: frontend-install

rebuild: down-vol build install start
