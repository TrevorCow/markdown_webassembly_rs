# Simple `comrak` Web Wrapper

A lightweight wrapper around the [`comrak`](https://github.com/kivikakk/comrak) library, enabling markdown to HTML
conversion in web applications using WebAssembly.

## 🚀 Getting Started

### Prerequisites

Ensure you have [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) installed, which is required for building
the WebAssembly module. If you don't have it installed, the included [
`cargo-make`](https://sagiegurari.github.io/cargo-make/) file can automate the installation for you.

You can easily install cargo-make using:

```bash
cargo install --force cargo-make
```

### 🛠️ Building

To build the project, you can use the provided `cargo make` tasks:

By default the development build builds for web, and the production build builds for a bundler. The different is for web
you have to initialize the WebAssembly yourself. See [index.html](test_website/index.html) for manual initialization.

- **Development Build:**

  Run the following command to build the project in development mode:

  ```bash
  cargo make
  ```

- **Production Build:**

  For a production-ready build, use:

  ```bash
  cargo make -p production
  ```

## 🌐 Usage

Integrate the generated WebAssembly module into your web application as follows:

```html
<script type="module">
    import {markdown_to_html} from "markdown_webassembly_rs";

    (async () => {
        // Sample markdown text
        const markdownText = "# Title\nHere is some markdown";

        // Convert markdown to HTML
        const htmlOutput = markdown_to_html(markdownText);

        // Inject the HTML into an article element
        const articleElement = document.createElement("article");
        articleElement.className = "markdown-body";
        document.body.appendChild(articleElement);
        articleElement.innerHTML = htmlOutput;
    })();
</script>
```

If using without a bundler you need to call init yourself:

```diff
<script type="module">
-   import {markdown_to_html} from "markdown_webassembly_rs";
+   import init, {markdown_to_html} from "markdown_webassembly_rs";

    (async () => {
+       await init();

        // Sample markdown text
        const markdownText = "# Title\nHere is some markdown";

        // Convert markdown to HTML
        const htmlOutput = markdown_to_html(markdownText);

        // Inject the HTML into an article element
        const articleElement = document.createElement("article");
        articleElement.className = "markdown-body";
        document.body.appendChild(articleElement);
        articleElement.innerHTML = htmlOutput;
    })();
</script>
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.