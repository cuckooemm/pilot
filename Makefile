PK_NAME=pilot
VERSION=0.1.0
DOCKER_REPO=cuckooemm

.PHONY: r c b build db-init push-image

all: r

mysql-init:
	mysql -h localhost -uroot -p cuckooemm entity/sql/mysql.sql
r :
	@cargo run
c:
	@cargo check
b: 
	@cargo build

build:
	docker build -t $(PK_NAME) .
tag:
	docker tag ${PK_NAME}:latest ${DOCKER_REPO}/${PK_NAME}:${VERSION}
push-image:
	docker push ${DOCKER_REPO}/${PK_NAME}:${VERSION}
