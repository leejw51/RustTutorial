all:
	cargo build
	cargo run --bin grpcserver &
	sleep 4
	cargo run --bin grpcclient 	
	sleep 1
	killall grpcserver
