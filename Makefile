IMAGES	:=	postgres adminer

CONTAINERS :=	postgres adminer

PATH_DB :=	./db

all:	detach frontend backend

build:
	mkdir -p ${PATH_DB}
	@docker compose  up --build
	docker ps

detach: 
	mkdir -p ${PATH_DB}
	@docker compose  up --build --detach
	docker ps

frontend:
	gnome-terminal -- npm run dev --prefix my-skeleton-app/

backend:
	gnome-terminal -- cargo watch -w src -C back-end -x run

clean: clean_volumes
	@docker compose  down -v
	@docker compose  stop
	-@docker rm -f ${CONTAINERS}
	-@docker rmi -f ${IMAGES}
	@docker volume rm -f `docker volume ls`

clean_volumes:
	rm -rf ${PATH_DB}

re: clean clean_volumes all

prune: 
	docker system prune -fa

.PHONY: all clean re build clean_volumes prune frontend backend detach
