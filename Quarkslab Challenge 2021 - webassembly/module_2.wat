(module

    (func $hello (import "" "hello"))
    (func $run_javascript (import "" "run_javascript")(param i32 i32)(result i32))

    (memory (export "memory") 2 3)

    (func (export "run_js")
        call $hello
        i32.const 0x1003
        call $lm
        i32.const 33
        call $run_javascript
        drop
        i32.const 0x1000
        i32.const 0xAA
        call $sm
        drop
    )

    (func (export "f1")(param $num1 i32)(param $num2 i32)(result i32)
        local.get $num1
        local.get $num2
        i32.add
    )


    (func (export "f2")(param $num1 i32)(param $num2 i32)(result i32)
        local.get $num1
        local.get $num2
        i32.sub
    )


    (func (export "f3")(param $num1 i32)(param $num2 i32)(result i32)
        local.get $num1
        local.get $num2
        i32.and
    )


    (func (export "f4")(param $num1 i32)(param $num2 i32)(result i32)
        local.get $num1
        local.get $num2
        i32.or
    )


    (func (export "f5")(param $num1 i32)(param $num2 i32)(result i32)
        local.get $num1
        local.get $num2
        i32.xor
    )


    (func (export "f6")(param $num1 i32)(param $num2 i32)(result i32)
        local.get $num1
        local.get $num2
        i32.rem_u
    )


    (func (export "f7")(param $num i32)(result i32)
        local.get $num
        i32.popcnt
    )


    (func (export "f8")(param $num i32)(result i32)
        local.get $num
        i32.const 1
        i32.rotl
    )


    (func (export "f9")(param $num i32)(result i32)
        local.get $num
        i32.const 1
        i32.rotr
    )

    (func (export "f10")(param $num i32)(result i32)
        local.get $num
        i32.const 0xff
        i32.xor
    )

    (func (export "f11")(result i32)
        i32.const 0x3000
        call $lm
    )


    (func (export "f12")(result i32)
        memory.size
    )


    (func $lm (param i32)(result i32)
        (i32.load8_u (local.get 0))
    )


    (func $sm(param $idx i32)(param $val i32)(result i32)
        (i32.store8 (local.get $idx)(local.get $val))
        i32.const 0
    )

    (data (i32.const 0x1000) "\01\02\03\04")
)
