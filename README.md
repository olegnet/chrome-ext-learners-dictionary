### This is an extension for Google Chrome

   The extension is in the early stage of development. Use it at your own risk.

### Build

1. Install [Rust](https://www.rust-lang.org/install.html)

2. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

   ```bash
   cargo install wasm-pack
   ```

3. Build

   ```bash
   wasm-pack build --target web --release
   ```

4. Build style.css

   ```bash
   npm install -D tailwindcss
   npm install daisyui
   npx tailwindcss -c ./tailwind.config.js -o ./pkg/style.css
   ```
   
   See also: [How to generate all classes in Tailwind CSS](https://design2tailwind.com/blog/tailwindcss-generate-all-classes/)


5. Run tests with [chromedriver](https://googlechromelabs.github.io/chrome-for-testing/)

   ```bash
   wasm-pack test --chrome --headless --chromedriver /path/to/chromedriver
   ```

6. Enable [developer mode in Chrome](chrome://extensions/) and load the extension


7. Go to [oxfordlearnersdictionaries.com](https://www.oxfordlearnersdictionaries.com/) and press Command-B/Ctrl-B to open the extension page


## License

    Copyright 2024 Oleg Okhotnikov

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
