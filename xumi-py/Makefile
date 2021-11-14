# 如果你的 BUILD_DIR 不同，可以 make BUILD_DIR=<your-dir>
BUILD_DIR := target/release

SRCS := $(wildcard src/*.rs) Cargo.toml
NAME = xunmi
TARGET = lib$(NAME)
# dylib 是 macos系统下的动态链接库格式
BUILD_FILE_D = $(BUILD_DIR)/$(TARGET).dylib 
BUILD_FILE1 = $(BUILD_DIR)/$(TARGET).so
TARGET_FILE = $(NAME)/$(NAME).so

all: $(TARGET_FILE)

test: $(TARGET_FILE)
	python3 -m pytest

$(TARGET_FILE): cargo_build $(BUILD_FILE1)
	@cp $(BUILD_FILE1) $(TARGET_FILE)

cargo_build: $(SRCS)
	@cargo build --release

$(BUILD_FILE1): mv_d_file

mv_d_file:
	@(if [ -f $(BUILD_FILE_D) ]; then echo "mv D file" & mv $(BUILD_FILE_D) $(BUILD_FILE1); fi)


clean: 
	cargo clean

PHONY: test cargo_build all