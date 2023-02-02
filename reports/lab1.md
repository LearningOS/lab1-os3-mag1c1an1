# 总结
分发syscall时更新计数桶，所有task第一次运行时记录get_time_us()，调用sys_task_info时，更新time。
在TaskManagerInner里使用Vec，如果用定长数组 lazy_static会一直spin很奇怪。感觉是.data的内存不够了。

# 回答
正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容 (运行 Rust 三个 bad 测例 (ch2b_bad_*.rs) ， 注意在编译时至少需要指定 LOG=ERROR 才能观察到内核的报错信息) ， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本

    StoreFault IllegalInstruction IllegalInstrucion ,
    对0x0写入 执行特权指令 
    RustSBI version 0.3.0-alpha.4

L40：刚进入 __restore 时，a0 代表了什么值。请指出 __restore 的两种使用情景。

    正常从__alltraps走下来的trap_handler流程。如果是这种情况，trap_handler会在a0里返回之前通过mv a0, sp传进去的&mut TrapContext。
    app第一次被__switch的时候通过__restore开始运行。这时候a0是个无关的数据（指向上一个TaskContext的指针）(切换task)

L46-L51：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

    sstatus sepc sscratch 
    sstatus 指示了返回的特权级
    sepc 记录 Trap 发生之前执行的最后一条指令的地址
    sscratch 是用户栈顶

L53-L59：为何跳过了 x2 和 x4？

    x2 是栈指针 现在sp是kernel stack 恢复之后得是user stack 现在内核栈使用 最后才能换sp
    x4 是 tp 现在用不到

L63：该指令之后，sp 和 sscratch 中的值分别有什么意义？

    sp 是 user stack
    sscratch 是 kernel stack

__restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

    sret， 因为sstatus被设置了user

L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？

    sp 是 kernel stack 
    sscratch 是 user stack
    
从 U 态进入 S 态是哪一条指令发生的？
    
    ecall