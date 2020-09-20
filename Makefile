
.PHONY: build clean runner libcheer

build: libhide-thread-atexit.so runner libcheer

runner:
	cd runner && cargo build

libcheer:
	cd libcheer && cargo build

clean:
	cd runner/ && cargo clean
	cd libcheer/ && cargo clean
	rm -f libhide-thread-atexit.so

libhide-thread-atexit.so: hide-thread-atexit.S
	gcc -shared -Wall hide-thread-atexit.S -o libhide-thread-atexit.so

run: build
	./runner/target/debug/runner ./libcheer/target/debug/libcheer.so

run-preload: build
	LD_PRELOAD=./libhide-thread-atexit.so ./runner/target/debug/runner ./libcheer/target/debug/libcheer.so

