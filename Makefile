./target/bundled/Dusk Phantom.vst3:
	cargo xtask bundle dusk_phantom --release

.PHONY: vst3
vst3: ./target/bundled/Dusk Phantom.vst3
	cp "./target/bundled/Dusk Phantom.vst3" ~/.vst3 -r
