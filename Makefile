default: debug

debug:
	cargo build
	qemu-system-x86_64 -bios /usr/share/OVMF/OVMF_CODE.fd 	\
	-enable-kvm -nographic -m 512                    \
    -device e1000,netdev=n0 	                            \
	-netdev user,bootfile=azphelos.efi,tftp=target/x86_64-unknown-uefi/debug,id=n0

run:
	cargo build --release
	qemu-system-x86_64 -bios /usr/share/OVMF/OVMF_CODE.fd 	\
	-enable-kvm -nographic -m 512                    \
    -device e1000,netdev=n0 	                            \
	-netdev user,bootfile=azphelos.efi,tftp=target/x86_64-unknown-uefi/release,id=n0
