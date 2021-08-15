import angr
import claripy
import time
import monkeyhex

import sys

#hook bypasing memory reallocation which causes state number explosion in angr
class wasm_rt_memory_grow_hook(angr.SimProcedure):
    def run(self):
        print('!!wasm_rt_memory_grow_hook called!!')
        return 0x0

#hook to determine value returend by op0
class op0_hook(angr.SimProcedure):
    def run(self):
        print('!!op0_hook called!!')
        retVal = claripy.BVS('op0_ret', 32);

        self.state.globals['op0_val'] = retVal
        return retVal

for inputLen in range(32,65):

    p = angr.Project('mainBin')

    p.hook_symbol('wasm_rt_grow_memory', wasm_rt_memory_grow_hook())
    p.hook_symbol('imp_op0', op0_hook())

    stdin_chars = [claripy.BVS('inChar_%d' % i, 8) for i in range(inputLen)]
    stdin_str = claripy.Concat(*stdin_chars + [claripy.BVV(b'\n')])


    st = p.factory.entry_state(args=["a.out"], stdin = stdin_str) #, add_options={"BYPASS_UNSUP
    # Constrain the first 32 bytes to be non-null and non-newline:
    i = 0
    for k in stdin_chars:
        st.solver.add(k < 0x7f)
        st.solver.add(k > 0x1F)

        i = i+1;


    sm = p.factory.simulation_manager(st)
    sm.use_technique(angr.exploration_techniques.MemoryWatcher())

    sm.explore(find=lambda s: b"You win!" in s.posix.dumps(1))

    print(" Run with inpust string length: " + str(inputLen) + " has ended ")

    if len(sm.found) > 0:
        print ("Weeee've got it")
        for s in sm.found:
            print("%s" % s.posix.dumps(1).decode("utf-8"))
            print("pass:")
            print("%s" % s.posix.dumps(0).decode("utf-8"))

        op0Solution = s.globals['op0_val']
        op0SolutionVal = s.solver.eval(op0Solution.reversed)

        print("op0 return value:")
        print(op0SolutionVal)
        exit()
    else:
        print("No solution found")


