default: run


run: cargo
	qemu-system-x86_64 -bios /usr/share/OVMF/OVMF_CODE.fd 	\
	-enable-kvm -nographic -m 1024                    \
    -device e1000,netdev=n0 	                            \
	-netdev user,bootfile=azphelos.efi,tftp=target/x86_64-unknown-uefi/debug,id=n0

cargo: 
	cargo build