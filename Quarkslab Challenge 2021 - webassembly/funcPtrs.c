#include <stdint.h>
#include <stddef.h>


uint32_t (*Z_envZ_op1Z_iii)(uint32_t, uint32_t) = NULL;
uint32_t (*Z_envZ_op2Z_iii)(uint32_t, uint32_t) = NULL;
uint32_t (*Z_envZ_op4Z_ii)(uint32_t)       = NULL;
uint32_t (*Z_envZ_op3Z_iii)(uint32_t, uint32_t) = NULL;
uint32_t (*Z_envZ_op0Z_iv)(void)      = NULL;

void (*Z_wasi_snapshot_preview1Z_proc_exitZ_vi)(uint32_t)                                     = NULL;
uint32_t (*Z_wasi_snapshot_preview1Z_args_sizes_getZ_iii)(uint32_t, uint32_t)                 = NULL;
uint32_t (*Z_wasi_snapshot_preview1Z_args_getZ_iii)(uint32_t, uint32_t)                       = NULL;
uint32_t (*Z_wasi_snapshot_preview1Z_fd_seekZ_iijii)(uint32_t, uint64_t, uint32_t, uint32_t)  = NULL;
uint32_t (*Z_wasi_snapshot_preview1Z_fd_closeZ_ii)(uint32_t)                                  = NULL;
uint32_t (*Z_wasi_snapshot_preview1Z_fd_fdstat_getZ_iii)(uint32_t, uint32_t)                  = NULL;
uint32_t (*Z_wasi_snapshot_preview1Z_fd_readZ_iiiii)(uint32_t, uint32_t, uint32_t, uint32_t)  = NULL;
uint32_t (*Z_wasi_snapshot_preview1Z_fd_writeZ_iiiii)(uint32_t, uint32_t, uint32_t, uint32_t) = NULL;

