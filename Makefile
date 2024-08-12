BASE_FONT=harfbuzz/base-fonts/OpenSans-Regular.ttf
TARGET_FONT=BrainFuck-Regular.ttf
WASM=brainfuck_bg.wasm

$(TARGET_FONT): pkg/$(WASM) $(BASE_FONT)
	harfbuzz/bin/otfsurgeon -i $(BASE_FONT) add -o $(TARGET_FONT) Wasm < pkg/$(WASM)

pkg/$(WASM): src/lib.rs
	wasm-pack build --target web

clean:
	rm -rf pkg/$(WASM) $(TARGET_FONT)
