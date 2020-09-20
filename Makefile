
.PHONY: build clean runner libcheer libnoop

build: libhide-thread-atexit.so runner libcheer libnoop

runner:
	cd runner && cargo build

libcheer:
	cd libcheer && cargo build

libnoop:
	cd libnoop && cargo build

clean:
	cd runner/ && cargo clean
	cd libcheer/ && cargo clean
	rm -f libhide-thread-atexit.so

libhide-thread-atexit.so: hide-thread-atexit.S
	gcc -shared -Wall hide-thread-atexit.S -o libhide-thread-atexit.so

gdb: 
	gdb --args ./runner/target/debug/runner ./libcheer/target/debug/libcheer.so

run: 
	./runner/target/debug/runner ./libcheer/target/debug/libcheer.so

run-preload: 
	LD_PRELOAD=./libhide-thread-atexit.so ./runner/target/debug/runner ./libcheer/target/debug/libcheer.so

valgrind: 
	valgrind --leak-check=full -- ./runner/target/debug/runner ./libcheer/target/debug/libcheer.so

valgrind-preload: 
	valgrind --trace-children=yes --leak-check=full -- env LD_PRELOAD=./libhide-thread-atexit.so ./runner/target/debug/runner ./libcheer/target/debug/libcheer.so

valgrind-preload2: 
	LD_PRELOAD=./libhide-thread-atexit.so valgrind --leak-check=full -- ./runner/target/debug/runner ./libcheer/target/debug/libcheer.so

