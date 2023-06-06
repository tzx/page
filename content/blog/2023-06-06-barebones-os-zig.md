+++
title = "Barebones RISC-V OS written in Zig"
+++

I am calling my OS `nosering`. The reason why is stupid and I am not going to explain. IYKYK. Here are my messy notes I written to bootstrap my OS.

### A Different Target

I used `zig init-exe` which allows building a compiled executable that will only run on my machine's native OS since the default compiled target would be for my machine (`aarch64-linux`). However, I want to compile my executable for the RISC-V architecture and have it run without an underlying operating system. To achieve this, I need to set the compilation target to `riscv64-freestanding` for my "freestanding" executable. Instead of running `zig build-exe` with the `-target riscv64-freestanding` flag to cross-compile, I decided to modify `build.zig` myself to gain a better understanding of Zig's build system and have a working binary with just `zig build`. The change to `build.zig` was simple:

```zig
// OLD CODE:
// const target = b.standardTargetOptions(.{});
// NEW CODE:
const target = std.zig.CrossTarget{
    .cpu_arch = std.Target.Cpu.Arch.x86,
    .os_tag = std.Target.Os.Tag.freestanding,
};
```

### Linker Script

I changed `main.zig` to only include `pub fn main() !void {}`. The starter code relies on using the Zig's standard library, but we cannot use it because we don't even have an operating system. We are making the operating system! When we try to compile and run `zig build`, we get these warnings:

```
$ zig build
LLD Link... warning(link): unexpected LLD stderr:
ld.lld: warning: cannot find entry symbol _start; not setting start address
```

We seemed to encounter a linking error, and that makes sense. Most of the time when we write programs, we compile them to run on our own machine, meaning it outputs binaries in a format that our operating system expects. The compiler automatically links our code to run on its host machine and the host machine's operating system would handle most of memory management. However, we don't have an OS to do that since we are writing the OS ourself. We need to write a custom linker script so we can define the memory layout of our binary, so we can load it into our hardware. Since we have no OS, we need to define our own entry point since we do not have a C runtime. By default the entry point is called `_start` but I changed it to `_enter`. We would then have the function `export fn _enter() callconv(.Naked) noreturn` in zig.

```
OUTPUT_ARCH("riscv")

/* Use _enter as our entry point to our program */
ENTRY(_enter)

MEMORY
{
  /* Source of ORIGIN: https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c#L97
   * Source of LENGTH: QEMU manual has -m default to be 128MiB
   */
  ram (rwxa): ORIGIN = 0x80000000, LENGTH = 128M
}


/* We are only loading from the file using PT_LOAD, we are not using dynamic linking */
PHDRS
{
  ram_fakerom PT_LOAD;
  ram_init PT_LOAD;
  ram PT_LOAD;
}

/* We are putting everything into RAM */
SECTIONS
{
  .text : ALIGN(4K) {
    *(.text.init);
    *(.text);
  } >ram :ram_fakerom

  .rodata : ALIGN(4K) {
    PROVIDE( _global_pointer = . );
    *(.rodata);
  } >ram :ram_fakerom

  .data : ALIGN(4K) {
    *(.data);
  } >ram :ram_init

  .bss : ALIGN(4K) {
    *(.bss);
  } >ram :ram

  PROVIDE( _bss_start = ADDR(.bss) );
  PROVIDE( _bss_end = ADDR(.bss) + SIZEOF(.bss) );

  PROVIDE( _stack_start = _bss_end );
  /* Stack size of 4kB */
  PROVIDE( _stack_end = _stack_start + 0x4000 );
  PROVIDE( _heap_start = _stack_end );
  PROVIDE( _PHYSTOP = ORIGIN(ram) + LENGTH(ram) );
}
```

After this I get a `R_RISCV_HI20 out of range` error when running `zig build`. I found [this](https://github.com/ziglang/zig/issues/5558) on GitHub and added `exe.code_model = .medium` to my `build.zig`. It compiled fine after. This makes sense because we start our program at `0x8000_0000` which lies outside the 2 GiB address range using `lui`.

We then add this linker script to executable in `build.zig` with `exe.setLinkerScriptPath(std.build.FileSource{ .path = "src/LINKERSCRIPT.ld" });`. After this, our code seems to compile without any errors.

### Running in QEMU

I won't be running my OS on real hardware (maybe later!), so I am using QEMU `virt` machine. You can list what hardware to emulate on using `qemu-system-riscv64 -machine help`. Since RISC-V machines are very different, images running on one machine likely will not run on another. Currently, I don't really care (honestly I don't really know) about particular hardware, so I am using `virt`. I also won't be loading any firmware and will just have QEMU just load the kernel I am building directly. To run my OS, I would just run `qemu-system-riscv64 -machine virt -bios none -kernel ./zig-out/bin/nosering`.

To see what's going on, we would have to run QEMU with added flags `-gdb tcp::1234 -S`, and then run `gdb` on our binary and then run the command `target remote :1234`. We should be able to debug and see that we enter the `_enter` function of our code.


### Okay, now what?

My `_enter` function is initially empty, so we are doing nothing. Secondly, we didn't set up our stack or statically allocated variables. These are usually done through assembly, so you should have the `_enter` function go to assembly to do these tasks and then go to Zig. You would add the assembly file with `exe.addAssemblyFile("src/ASMFILE.S");`. My assembly file looks like this:

```asm
.section .text.init

.global _enter
_enter:
    # Only make the guaranteed hardware thread (hart) of id 0 do bootstrapping
    # while the rest just wait for interrupts
    csrr        t0, mhartid
    bnez        t0, wait_for_interrupt

    # Don't do any address translation or protection
    csrw        satp, zero

# https://www.sifive.com/blog/all-aboard-part-3-linker-relaxation-in-riscv-toolchain
.option push
.option norelax
    la      gp, _global_pointer
.option pop
    
    # Set up the stack
    la      sp, _stack_end

    # Clear the bss section; it is expected to be zero
    la      t5, _bss_start
    la      t6, _bss_end
bss_clear:
    sd      zero, (t5)
    addi    t5, t5, 8
    bltu    t5, t6, bss_clear
3:
    # interrupts later
    # la      t1, kmain
    # csrw    mepc, t1
    tail kmain

wait_for_interrupt:
    wfi
    j wait_for_interrupt
```

We would change `_enter` to `kmain` in `src/main.zig`.

### Global pointer relaxation

Instead of having an `auipc` instruction, we can save a global pointer symbol in our linker and use that symbol in our assembly instruction to address relative to that symbol rather than using `auipc` to help calculate some global address. This is called global pointer relaxation. However, we must first initially disable this feature first and set the global pointer here:

```asm
.option push
.option norelax
    la      gp, _global_pointer
.option pop
```

The reason we need to disable relaxation is because this will relax to `mv gp, gp` but we have not set up `gp` to do relaxation. There is no relative address if there is no address to relate to.

### Naked!

When I changed `_enter` to `kmain`, I was wondering why I was not entering `kmain` correctly when debugging. This is because I was using `callconv(.Naked)`! to ignore any calling conventions. I have a stack, but I am not setting anything up due to having my function be naked. This caused me to reference bad addresses that were not set up. Changing it to a standard calling convention fixed this bug. The reason `callconv(.Naked)` was there in the first place is because that function (before the name change) was originally our entry point where we should not be using a calling convention.

### DTC

I was looking through QEMU's source code to find the address, `0x8000_0000`, corresponding to RAM to load our code in. I was wondering if there's a way to do it without looking through code. Luckily, there's a tool called `dtc` to read a QEMU machine's devicetree blob. We can obtain the blob by running `qemu-system-riscv64 -machine virt -machine dumpdtb=virt.dtb`. We then would be able to convert it to a human-readable format using `dtc -I dtb -O dts -o virt.dts virt.dtb`. If you read `virt.dts`, you should be able see the `memory@80000000` section where the `reg` value would tell us the size and length section:

```
memory@80000000 {
        device_type = "memory";
        reg = <0x00 0x80000000 0x00 0x8000000>;
};
```

This tells us that our machine's memory address starts at `0x8000_0000` and is `0x8000_0000` bytes (`128M`) long. You can also read the `virt.dts` file to inspect other hardware elements such as the `UART` which is memory mapped at `0x1000_0000`.

```
uart@10000000 {
            interrupts = <0x0a>;
            interrupt-parent = <0x02>;
            clock-frequency = <0x384000>;
            reg = <0x00 0x10000000 0x00 0x100>;
            compatible = "ns16550a";
};
```

### Hello World!

We can write to our UART address and connect our serial port to stdio to see a message in our console. Add the flags to qemu: `-serial mon:stdio`. We use `*volatile` since we are expecting side effects because the address we are referencing is memory mapped.

```zig
export fn kmain() noreturn {
    var uart = @intToPtr(*volatile u8, 0x1000_0000);
    for ("hello world!") |c| {
        uart.* = c;
    }
    while (true) {}
}
```

### Summary

- QEMU command: `qemu-system-riscv64 -machine virt -bios none -kernel ./zig-out/bin/nosering -nographic -serial mon:stdio`
- QEMU command + debug: `qemu-system-riscv64 -machine virt -bios none -kernel ./zig-out/bin/nosering -nographic -serial mon:stdio -gdb tcp::1234 -S`
- Source code: [https://github.com/tzx/nOSering/tree/d1ceaf90e00338de95359e655d97e3143365c229](https://github.com/tzx/nOSering/tree/d1ceaf90e00338de95359e655d97e3143365c229)
