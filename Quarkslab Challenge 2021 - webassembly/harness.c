/*
	This is a harness to emulate uvwassi calls used by challenge and make it more
	digestible for angr.
	This his very horrible piece of code done just to make
	the challenge running. Please don't try to reuse this code it for anything.
*/

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdlib.h>

#include <unistd.h>
#include <sys/uio.h>


#include "uv.h"

#include "wasm-rt.h"

//#define CALL_INFO printf("%s calledn !\n", __FUNCTION__)
#define CALL_INFO


typedef uint8_t uvwasi_whence_t;
#define UVWASI_WHENCE_SET 0
#define UVWASI_WHENCE_CUR 1
#define UVWASI_WHENCE_END 2

typedef uint64_t __wasi_rights_t;
typedef uint16_t __wasi_fdflags_t;
typedef uint8_t __wasi_filetype_t;

typedef struct __wasi_fdstat_t {
  __wasi_filetype_t fs_filetype;
  __wasi_fdflags_t fs_flags;
  __wasi_rights_t fs_rights_base;
  __wasi_rights_t fs_rights_inheriting;
} __wasi_fdstat_t;

typedef uint32_t __wasi_size_t;
typedef uint32_t __wasi_ptr_t;

typedef struct __wasi_iovec_t {
  __wasi_ptr_t buf;
  __wasi_size_t buf_len;
}__wasi_iovec_t;


static uint32_t  __argc;
static uint8_t **__argv;

static FILE** fd_tab = NULL;
static uint32_t fd_tab_size = 0;


static wasm_rt_memory_t* module_memory;

void harnessSetup(int argc, char **argv, FILE** fds, uint32_t fdNum, wasm_rt_memory_t* memPtr)
{

    fd_tab = fds;
    fd_tab_size = fdNum;

  __argc = argc > 1 ? argc-1 : 0;
  __argv = argc > 1 ? (uint8_t**)&argv[1] : NULL;

   module_memory = memPtr;
}


uint8_t* getMemPtr(uint32_t target_addr)
{
    /*TODO check access */
    return module_memory->data + target_addr;
}

uint32_t writeValue(uint8_t* valPtr, uint32_t target_addr, size_t data_size)
{
    uint8_t* abs_address = getMemPtr(target_addr);
    memcpy((void*)abs_address, (void*)valPtr, data_size);
    return 0x0;

}

void imp_proc_exit(uint32_t arg0 )
{
    exit(arg0);
    return;
}

uint32_t imp_args_sizes_get(uint32_t num_addr, uint32_t size_addr)
{
    CALL_INFO;
    uint32_t argc = __argc;

    uint32_t arg_buf_size = 0;
    for(uint32_t i = 0 ; i < __argc; i++)
    {
         arg_buf_size += strlen(__argv[i]) + 1; //include 0x0 at the end
    }

    writeValue((uint8_t*)&argc, num_addr, sizeof(uint32_t));
    writeValue((uint8_t*)&arg_buf_size, size_addr, sizeof(uint32_t));

    return 0x0;
}

uint32_t imp_args_get(uint32_t wasm_argv, uint32_t wasm_buf)
{
    CALL_INFO;
    uint32_t argc = __argc;

    uint32_t arg_buf_size = 0;
    for(uint32_t i = 0 ; i < __argc; i++)
    {
         arg_buf_size += strlen(__argv[i])+1; //include 0x0 at the end
    }

    uint8_t* tgt_buf = getMemPtr(wasm_buf);

    for (struct{uint32_t i; uint32_t offset; }loopy = {0,0}; loopy.i < argc; loopy.i++)
    {
        uint32_t size = strlen(__argv[loopy.i])+1; //include 0x0 at the end
        memcpy(tgt_buf+loopy.offset, __argv[loopy.i], size);

        uint32_t rel_address = wasm_buf + loopy.offset ;
        uint32_t dest = wasm_argv + (loopy.i * sizeof(uint32_t));
        writeValue((uint8_t*)&rel_address, dest, sizeof(uint32_t));

        loopy.offset += size;
    }


    return 0x0;
}

uint32_t imp_fd_seek(uint32_t fd, uint64_t offset, uint32_t whence, uint32_t newoffset_addr)
{
    CALL_INFO;
    int real_whence;

    if(fd > fd_tab_size){ return -1;}
    if(fd_tab[fd] == NULL){ return -1;}
    FILE* fileObj = fd_tab[fd];

    if (whence == UVWASI_WHENCE_CUR)
      real_whence = SEEK_CUR;
    else if (whence == UVWASI_WHENCE_END)
      real_whence = SEEK_END;
    else if (whence == UVWASI_WHENCE_SET)
      real_whence = SEEK_SET;
    else
      return -1;

    off_t r = lseek(fileno(fileObj), offset, real_whence);
    if( -1 == r)
    { return -1; }

    uint64_t newOffset = r;
    writeValue((uint8_t*)&newOffset, newoffset_addr, sizeof(uint64_t));

    return r;

}

uint32_t imp_fd_close(uint32_t fd)
{
    CALL_INFO;
    return 0;
}

uint32_t imp_fd_fdstat_get(uint32_t fd, uint32_t stat_ptr )
{
    CALL_INFO;

     __wasi_fdstat_t* statbuf = ( __wasi_fdstat_t*)getMemPtr(stat_ptr);
     memset(statbuf,0x0,sizeof(__wasi_fdstat_t));
/*     statbuf->fs_filetype = 0x2;
     statbuf->fs_flags = 0x8002;
     statbuf->fs_rights_base = 0x820004a;
     statbuf->fs_rights_inheriting = 0x0;
*/
    return 0;

}

uint32_t imp_fd_read(uint32_t fd, uint32_t iovptr, uint32_t iovcnt, uint32_t out_ptr)
{
    CALL_INFO;

    if(fd > fd_tab_size){ return -1;}
    if(fd_tab[fd] == NULL){ return -1;}
    FILE* fileObj = fd_tab[fd];
 
    if(0u == iovcnt)
    { return -1; }

    __wasi_iovec_t* wasm_iovs = (__wasi_iovec_t*)getMemPtr(iovptr);

    ssize_t result = 0;
    if(iovcnt == 1)
    {
        __wasi_size_t chunk_size = wasm_iovs[0].buf_len;
        uint8_t* mem_bufAddr = getMemPtr(wasm_iovs[0].buf);

        result = read(fileno(fileObj), mem_bufAddr, chunk_size);

    }else{
        struct iovec* bufVec = malloc(iovcnt * sizeof(struct iovec));
        for(uint32_t i = 0; i < iovcnt;i++)
        {
            bufVec[i].iov_len = wasm_iovs[i].buf_len;
            bufVec[i].iov_base =  getMemPtr(wasm_iovs[i].buf);
        }

        result = readv(fileno(fileObj), bufVec, iovcnt);


        free(bufVec);
    }

    if( -1 != result){
        __wasi_size_t bytesNum = result;
        writeValue((uint8_t*)&bytesNum, out_ptr, sizeof(__wasi_size_t));
    }

    return (-1 == result) ? -1 : 0;
}

uint32_t imp_fd_write(uint32_t fd, uint32_t iovptr, uint32_t iovcnt, uint32_t out)
{
    CALL_INFO;
    __wasi_iovec_t* wasm_iovs = (__wasi_iovec_t*)getMemPtr(iovptr);
    __wasi_ptr_t* out_addr = (__wasi_ptr_t*)getMemPtr(out);

    *out_addr = 0;

    if(fd > fd_tab_size){ return -1;}
    if(fd_tab[fd] == NULL){ return -1;}
    FILE* fileObj = fd_tab[fd];

    size_t w;
    __wasi_size_t numWritten = 0;
    for(uint32_t i = 0; i < iovcnt; i++)
    {
        uint8_t* mem_addr = getMemPtr(wasm_iovs[i].buf);
        w = write(fileno(fileObj), mem_addr, wasm_iovs[i].buf_len);

        if(-1 == w)
            break;

        numWritten += w;

    }

    writeValue((uint8_t*)&numWritten, out, sizeof(numWritten));

    return (-1 == w) ? -1 : 0;

}
