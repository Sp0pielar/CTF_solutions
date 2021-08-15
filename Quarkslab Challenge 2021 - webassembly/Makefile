CFLAGS = -ggdb3 
INCLUDE = -I./ -I./wabt/wasm2c/ -I./wabt/third_party/uvwasi/include/  -I./wabt/out/gcc/Debug/_deps/libuv-src/include
LIBS   = ./wabt/build/libwasm-rt-impl.a -lpthread -ldl
CC     = gcc
WASM_F = module_1
OBJS   = $(WASM_F).o harness.o funcPtrs.o main.o 


.PHONY: wabt clean part_clean all wasm_to_c

all: wabt mainBin


wabt:
	sed -i '/#define\sWASM_RT_MEMCHECK_SIGNAL_HANDLER\s1/ s/1/0/' wabt/wasm2c/wasm-rt.h
	./buildwabt.sh

$(WASM_F).wasm:
	dd if=chall1.exe of=$(WASM_F).wasm bs=1 skip=5804208 count=32596
	dd if=chall1.exe of=module_2.wat bs=1 skip=5836816 count=2251

$(WASM_F).c: $(WASM_F).wasm
	./wabt/build/wasm2c $(WASM_F).wasm -o $(WASM_F).c

%.o: %.c
	$(CC) -c -o $@ $< $(CFLAGS) $(INCLUDE)

mainBin: $(OBJS)
	$(CC) $(CFLAGS) -o $@ $(OBJS) $(LIBS)

mainBinClean:
	rm -f *.o
	rm -f mainBin
	rm -f $(WASM_F).c $(WASM_F).h

clean: mainBinClean
	rm -rf wabt/build
	rm -rf wabt/bin
