# rustdedup
Deduplicate files from stdin at rust speeds, output in stdout.

### Memory optimizations made:
* Well rust.
* Input lines directly streamed to the processing threads without collecting them all first.
* Partitions the hash space to reduce lock contention.

### Some stats
In the below test we utilise a small 75mb file (_else we wait too long for hyperfine_) with 1 595 966 lines of data.
![image](https://github.com/InitRoot/rustdedup/assets/954507/ba3f52ac-ffdb-4ad7-8f4c-25b6c848bb05)

When we up the anty a little bit going to large files 2.3gb we see some improvements.
![image](https://github.com/InitRoot/rustdedup/assets/954507/196c6426-bdc2-4c9a-b9cb-99b0b6b788ea)

When we compare with the likes of duplicut (https://github.com/nil0x42/duplicut) some significant improvements can be seen, however, I'm not sure if this boils down to the rust usage over c.
![image](https://github.com/InitRoot/rustdedup/assets/954507/962504a0-a685-43f4-8b42-bc636d46e7df)


### Usage
```
cat file.txt | rustdedup

rustdedup -i /diska9.txtextra.csvmodded.csv -o output2.txt
```
