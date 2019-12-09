# Cpu Memory Survey

## 参考博客

* https://fosschef.wordpress.com/2011/07/08/prefetch-performance-and-toxic/
* http://igoro.com/archive/gallery-of-processor-cache-effects/
* https://www.aristeia.com/TalkNotes/ACCU2011_CPUCaches.pdf
* CSAPP, 3edtion.

## Tesing Machine:

我的台式，使用 `lscpu`

```

架构：                           x86_64
CPU 运行模式：                   32-bit, 64-bit
字节序：                         Little Endian
Address sizes:                   43 bits physical, 48 bits virtual
CPU:                             16
在线 CPU 列表：                  0-15
每个核的线程数：                 2
每个座的核数：                   8
座：                             1
NUMA 节点：                      1
厂商 ID：                        AuthenticAMD
CPU 系列：                       23
型号：                           113
型号名称：                       AMD Ryzen 7 3800X 8-Core Processor
步进：                           0
Frequency boost:                 enabled
CPU MHz：                        1961.707
CPU 最大 MHz：                   3900.0000
CPU 最小 MHz：                   2200.0000
BogoMIPS：                       7802.74
虚拟化：                         AMD-V
L1d 缓存：                       256 KiB
L1i 缓存：                       256 KiB
L2 缓存：                        4 MiB
L3 缓存：                        32 MiB
NUMA 节点0 CPU：                 0-15
Vulnerability Itlb multihit:     Not affected
Vulnerability L1tf:              Not affected
Vulnerability Mds:               Not affected
Vulnerability Meltdown:          Not affected
Vulnerability Spec store bypass: Mitigation; Speculative Store Bypass disabled via prctl and seccomp
Vulnerability Spectre v1:        Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:        Mitigation; Full AMD retpoline, IBPB conditional, STIBP conditional, RSB filling
Vulnerability Tsx async abort:   Not affected
标记：                           fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonst
                                 op_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a
                                 misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbas
                                 e bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr w
                                 bnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca

```

The message above is shown by `lscpu`.

---

我的 Mac，使用 `sysctl hw`

```
hw.ncpu: 8
hw.byteorder: 1234
hw.memsize: 17179869184
hw.activecpu: 8
hw.physicalcpu: 4
hw.physicalcpu_max: 4
hw.logicalcpu: 8
hw.logicalcpu_max: 8
hw.cputype: 7
hw.cpusubtype: 8
hw.cpu64bit_capable: 1
hw.cpufamily: 280134364
hw.cacheconfig: 8 2 2 8 0 0 0 0 0 0
hw.cachesize: 17179869184 32768 262144 6291456 0 0 0 0 0 0
hw.pagesize: 4096
hw.pagesize32: 4096
hw.busfrequency: 100000000
hw.busfrequency_min: 100000000
hw.busfrequency_max: 100000000
hw.cpufrequency: 2200000000
hw.cpufrequency_min: 2200000000
hw.cpufrequency_max: 2200000000
hw.cachelinesize: 64
hw.l1icachesize: 32768
hw.l1dcachesize: 32768
hw.l2cachesize: 262144
hw.l3cachesize: 6291456
hw.tbfrequency: 1000000000
hw.packages: 1
hw.optional.floatingpoint: 1
hw.optional.mmx: 1
hw.optional.sse: 1
hw.optional.sse2: 1
hw.optional.sse3: 1
hw.optional.supplementalsse3: 1
hw.optional.sse4_1: 1
hw.optional.sse4_2: 1
hw.optional.x86_64: 1
hw.optional.aes: 1
hw.optional.avx1_0: 1
hw.optional.rdrand: 1
hw.optional.f16c: 1
hw.optional.enfstrg: 1
hw.optional.fma: 1
hw.optional.avx2_0: 1
hw.optional.bmi1: 1
hw.optional.bmi2: 1
hw.optional.rtm: 0
hw.optional.hle: 0
hw.optional.adx: 0
hw.optional.mpx: 0
hw.optional.sgx: 0
hw.optional.avx512f: 0
hw.optional.avx512cd: 0
hw.optional.avx512dq: 0
hw.optional.avx512bw: 0
hw.optional.avx512vl: 0
hw.optional.avx512ifma: 0
hw.optional.avx512vbmi: 0
hw.targettype: Mac
hw.cputhreadtype: 1
```

## Surveys

### Cache Friendly Code

```bash
cargo bench
```

`benches/cache_benchmark.rs` . 感觉都是n倍了，编译器加把劲优化啊

### CPU Cache Line

```bash
cargo run --release
```

`src/main.rs` 一开始跑了半个多小时效果都不好，后来换到 Intel i7 的 Mac 上效果显著增长。

可能是 Amd 做了什么优化...?

 
