FLATPAK_MANIFEST = org.margual56.NixBucks.yml
FLATPAK_TARGET = ./target/flatpak

check:
	cargo check || exit 2

pre: check
	sudo systemctl start docker
 
windows: pre
	cross build --target-dir windows/ --release --target x86_64-pc-windows-gnu || exit 1
	mv windows/x86_64-pc-windows-gnu/release/nix-bucks.exe ./nix-bucks.exe

linux: check
	cargo build --target-dir linux/ --release 
ifneq ("$(wildcard $(nix-bucks))","")
		mv -f linux/release/nix-bucks ./nix-bucks
endif

clean: 
	rm -rf linux windows

flatpak: linux 
	flatpak-builder --force-clean $(FLATPAK_TARGET) $(FLATPAK_MANIFEST)

flatpak-install: flatpak
	flatpak-builder --user --install --force-clean $(FLATPAK_TARGET) $(FLATPAK_MANIFEST)
	flatpak run org.margual56.NixBucks &

.PHONY: pre clean flatpak flatpak-install linux windows check
