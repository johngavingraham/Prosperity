# Prosperity
A quick, shoddy and unsafe submission to https://www.mattkeeter.com/projects/prospero

![](out.png)

### Timing

```
❯ time target/release/prosperity
target/release/prosperity  58.84s user 0.02s system 1524% cpu 3.861 total
```


## Optimize lol

```
$ git checkout main
Switched to branch 'main'

$ rm out.png && cargo build --release && time ./target/release/prosperity && md5sum out.png
    Finished `release` profile [optimized] target(s) in 0.03s

real	0m16.958s
user	2m12.821s
sys	0m0.025s
173d8cf7b35cb6077524fc0393913791  out.png

$ git checkout optimizelol
Switched to branch 'optimizelol'

$ rm out.png && cargo build --release && time ./target/release/prosperity && md5sum out.png
   Compiling vm-parser v0.1.0 (/home/john/tmp/prospero/sam/Prosperity/vm-parser)
   Compiling prosperity v0.1.0 (/home/john/tmp/prospero/sam/Prosperity)
    Finished `release` profile [optimized] target(s) in 2.95s

real	0m1.427s
user	0m11.135s
sys	0m0.009s
173d8cf7b35cb6077524fc0393913791  out.png
```
