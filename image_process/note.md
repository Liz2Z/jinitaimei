# ffmpeg 指令

1. 抽帧

```bash
ffmpeg -i 鸡你太美.flv -vsync 0 -vf select='not(mod(n\, 14))',showinfo output/%08d.jpg
```

2. 查看视频帧数

```bash
ffprobe -v error -count_frames -select_streams v:0 -show_entries stream=nb_read_frames -of default=nokey=1:noprint_wrappers=1 鸡你太美.flv
```
