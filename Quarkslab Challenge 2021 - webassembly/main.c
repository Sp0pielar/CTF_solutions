#include <assert.h>
#include "uv.h"
#include "uvwasi.h"

#include "wasm-rt.h"

#include "module_1.h"

extern void harnessSetup(int argc, char **argv, FILE** fds, uint32_t fdNum, wasm_rt_memory_t* memPtr);

extern void imp_proc_exit(uint32_t );
extern uint32_t imp_args_sizes_get(uint32_t, uint32_t );
extern uint32_t imp_args_get(uint32_t, uint32_t );
extern uint32_t imp_fd_seek(uint32_t, uint64_t, uint32_t, uint32_t);
extern uint32_t imp_fd_close(uint32_t);
extern uint32_t imp_fd_fdstat_get(uint32_t, uint32_t );
extern uint32_t imp_fd_read(uint32_t, uint32_t, uint32_t, uint32_t);
extern uint32_t imp_fd_write(uint32_t, uint32_t, uint32_t, uint32_t);

static uint32_t imp_op1(uint32_t, uint32_t);
static uint32_t imp_op2(uint32_t, uint32_t);
static uint32_t imp_op4(uint32_t);
static uint32_t imp_op3(uint32_t, uint32_t);
static uint32_t imp_op0(void);


void setHarnessFuncPtrs()
{
    Z_wasi_snapshot_preview1Z_proc_exitZ_vi       = &imp_proc_exit;
    Z_wasi_snapshot_preview1Z_args_sizes_getZ_iii = &imp_args_sizes_get;
    Z_wasi_snapshot_preview1Z_args_getZ_iii       = &imp_args_get;
    Z_wasi_snapshot_preview1Z_fd_seekZ_iijii      = &imp_fd_seek;
    Z_wasi_snapshot_preview1Z_fd_closeZ_ii        = &imp_fd_close;
    Z_wasi_snapshot_preview1Z_fd_fdstat_getZ_iii  = &imp_fd_fdstat_get;
    Z_wasi_snapshot_preview1Z_fd_readZ_iiiii      = &imp_fd_read;
    Z_wasi_snapshot_preview1Z_fd_writeZ_iiiii     = &imp_fd_write;
}

void initfuncPtrs()
{

    Z_envZ_op1Z_iii = &imp_op1;
    Z_envZ_op2Z_iii = &imp_op2;
    Z_envZ_op4Z_ii  = &imp_op4;
    Z_envZ_op3Z_iii = &imp_op3;
    Z_envZ_op0Z_iv  = &imp_op0;
}



int main(int argc, char **argv)
{

    FILE* _t[] = {stdin, stdout, stderr};

    (*WASM_RT_ADD_PREFIX(init))();

    harnessSetup(argc, argv, _t, 3, WASM_RT_ADD_PREFIX(Z_memory));

    setHarnessFuncPtrs();
    initfuncPtrs();

    (*WASM_RT_ADD_PREFIX(Z__startZ_vv))();

   return 0;
}


uint32_t _rotl(uint32_t value, int32_t shift) {
    if ((shift &= 31) == 0)
      return value;
    return (value << shift) | (value >> (32 - shift));
}

uint32_t _rotr(uint32_t value, int32_t shift) {
    if ((shift &= 31) == 0)
      return value;
    return (value >> shift) | (value << (32 - shift));
}

/* functions emulating calls to the second module*/
/* reversed from the challenge binary sub_140511600 */
static uint32_t imp_op1(uint32_t a1, uint32_t a2)
{
    return a1 ^ a2;
}

static uint32_t imp_op2(uint32_t a1, uint32_t a2)
{
    uint32_t val = a1;
    for(uint32_t i = 0; i < a2; i++)
    {
        val = _rotl(val, 1);
        val = ( ( val >> 0x8 )| val & 0xff);
    }
    return val;
}

static uint32_t imp_op4(uint32_t a1)
{
      return a1 ^ 0xff;
}

static uint32_t imp_op3(uint32_t a1, uint32_t a2)
{
    uint32_t val = a1;
    for(uint32_t i = 0; i < a2; i++)
    {
        val = _rotr(val, 1);
        val = ( ( val >> 0x18 ) & 0xffffff80) | val;
        val &= 0xff;
    }
    return val;

}

//in the challenge binary this will call GetConsoleScreenBufferInfo.
static uint32_t imp_op0(void)
{
    //return 238u; //This is actual return value that is needed for wasm to continue
                   //In the challenge binary GetConsoleScreenBufferInfo should return 0xEE
    return 0;
}


