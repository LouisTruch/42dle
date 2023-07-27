IMAGES	:=	postgres back

CONTAINERS :=	postgres back-end

PATH_DB :=	./db

all:	build

build:
	mkdir -p ${PATH_DB}
	@docker compose  up --build
	docker ps

stop:
	@docker compose  stop

down:
	@docker compose  down -v

clean: down stop clean_volumes
	-@docker rm -f ${CONTAINERS}
	-@docker rmi -f ${IMAGES}
	@docker volume rm -f `docker volume ls`

clean_volumes:
	rm -rf ${PATH_DB}

re: clean clean_volumes all

prune: 
	docker system prune -fa

.PHONY: all clean re build stop down clean_volumes prune
