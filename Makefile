.PHONY: build_backend_image
all: build_backend_image
#.DEFAULT_GOAL := clean
build_backend_image:
	@echo "Backend image is building"
	@docker build --tag backend:latest .
	@echo "Backend image has been build"