# bell

Windows用的極簡CLI音效播放程式，以Rust實作。
可銜接於其他指令的尾部，實現前面的指令完成時播放聲音通知。

## 使用方式

不帶參數時，播放內嵌於EXE中的`Aria_task_finished.wav`：

```cmd
bell.exe
```

指定WAV檔案時，播放指定的音效：

```cmd
bell.exe "C:\path\to\sound.wav"
```

程式會等待音效播放完成後才結束。

## 特點

- Windows原生CLI程式
- Rust實作
- 不需要額外安裝Runtime

## 編譯

需要Rust工具鏈。

```cmd
cargo build --release
```

## License

請參閱[LICENSE](LICENSE)。
