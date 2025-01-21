.PHONY: clean build run check

clean:
	@clear
	@echo "🫧 Cleaning up..."
	@rm -rf target
	@rm -f ./0-shell
	@echo "✅  Cleaned up."

build:
	@clear
	@make clean
	@echo "🚀 Building..."
	@cargo build --release
	@mv target/release/shell ./0-shell
	@echo "✅  Built."

run:
	@clear
	@echo "🛫 0-Shell is Running..."
	@./0-shell

check:
	@clear
	@echo "🧪 Checking..."
	@cargo check
	@echo "✅  Checked."