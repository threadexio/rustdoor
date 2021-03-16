ARGS = -j8

TARGETS =	x86_64-unknown-linux-musl \
			x86_64-unknown-linux-gnu \
			x86_64-pc-windows-gnu \

BUILD_CMD = cargo build $(ARGS) --target

# Release build
.PHONY: release
release: clean
	for t in $(TARGETS); do \
		$(BUILD_CMD) $$t --release; \
	done

# Debug build
.PHONY: debug
debug: clean
	for t in $(TARGETS); do \
		$(BUILD_CMD) $$t; \
	done

# Install the given targets
.PHONY: install
install:
	for t in $(TARGETS); do \
		rustup target add $$t; \
	done

# Clean the target directory
.PHONY: clean
clean:
	@cargo clean
