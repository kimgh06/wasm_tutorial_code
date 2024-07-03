컴파일러 설치

```bash
cargo install wasm-pack
```

컴파일

```bash
wasm-pack build --target web
```

서버 실행

8000은 원하는 포트 번호로 변경이 가능합니다.
```bash
python -m http.server 8000
```
