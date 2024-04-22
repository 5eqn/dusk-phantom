.PHONY: vst3 win

build:
	cargo xtask bundle dusk_phantom --release

build-win:
	cross xtask bundle dusk_phantom --release --target x86_64-pc-windows-gnu

vst3: build
	cp "./target/bundled/Dusk Phantom.vst3" ~/.vst3 -r

win: build-win
	mv "./target/bundled/Dusk Phantom.vst3/Contents/x86_64-win/Dusk Phantom.vst3" "/mnt/win/c/Program Files/Common Files/VST3/"
