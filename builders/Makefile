BUILDERS_PREFIX := initia/go-ext-builder:0001

.PHONY: docker-image-debian
docker-image-debian:
	docker build --pull . -t $(BUILDERS_PREFIX)-debian -f ./Dockerfile.debian

.PHONY: docker-image-cross
docker-image-cross:
	docker build --pull . -t $(BUILDERS_PREFIX)-cross -f ./Dockerfile.cross

.PHONY: docker-image-alpine
docker-image-alpine:
	docker build --pull . -t $(BUILDERS_PREFIX)-alpine -f ./Dockerfile.alpine

.PHONY: docker-images
docker-images: docker-image-debian docker-image-cross docker-image-alpine

.PHONY: docker-publish
docker-publish: docker-images
	docker push $(BUILDERS_PREFIX)-cross
	docker push $(BUILDERS_PREFIX)-debian
	docker push $(BUILDERS_PREFIX)-alpine
