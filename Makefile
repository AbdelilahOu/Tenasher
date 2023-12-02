dev: 
	cargo run
	
containerup:
	docker run --name test -e POSTGRES_USER=root -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres:15

containerdown:
	docker stop test
	docker rm --force test

createdb: 
	docker exec -it test createdb --username=root --owner=root school

dropdb:
	docker exec -it test dropdb school

mup:
	sea-orm-cli migrate up

mdownlatest:
	sea-orm-cli migrate down

mdownall: 
	sea-orm-cli migrate reset

mdownfresh:
	sea-orm-cli migrate fresh

mdownrefresh:
	sea-orm-cli migrate refresh

entity:
	sea-orm-cli generate entity -o entity/src --lib

.PHONY: createdb dropdb mup mdownlatest mdownall mdownfresh mdownrefresh entity
