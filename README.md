# bell

Windows用的極簡CLI音效播放程式，以Rust實作。

## 使用方式

不帶參數時，播放內嵌於EXE中的`Aria_task_finished.wav`：

```cmd
bell.exe
```

指定WAV檔案時，播放指定的音效：

```cmd
bell.exe "C:\\path\\to\\sound.wav"
```

程式會等待音效播放完成後才結束。

## 特點

- Windows原生CLI程式
- Rust實作
- 預設音效`Aria_task_finished.wav`於編譯時直接嵌入EXE
- 使用repo中的`bell.ico`作為Windows程式圖示
- 不需要額外安裝Runtime

## 編譯

需要Rust工具鏈。

```cmd
cargo build --release
```

Windows x64版本也會由GitHub Actions自動編譯。

## License

請參閱[LICENSE](LICENSE)。
