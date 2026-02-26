# Mi Proyecto
# Pagina Web 2 — Proyecto Rust + WebAssembly

Este proyecto es una aplicación web desarrollada en **Rust**, compilada a **WebAssembly (Wasm)** y construida con el ecosistema moderno para aplicaciones web reactivas. Su objetivo es ofrecer una base sólida para crear interfaces web rápidas, seguras y eficientes utilizando Rust en lugar de JavaScript.

---

## 🚀 Tecnologías utilizadas

- **Rust** — Lenguaje principal del proyecto.
- **WebAssembly (Wasm)** — Para ejecutar Rust en el navegador.
- **wasm-bindgen** — Puente entre Rust y APIs del navegador.
- **web** — Framework para construir interfaces web reactivas en Rust.
- **web-sys** — Acceso directo a APIs del DOM.
- **serde** — Serialización y deserialización de datos.
- **gloo** — Utilidades para desarrollo web en Rust.
- **wasm-bindgen-futures** — Manejo de `async`/`await` en Wasm.

---

## 📦 Estructura del proyecto

---

## 🛠️ Instalación y ejecución

### 1. Instalar las herramientas necesarias

Asegúrate de tener instalado:

- Rust (nightly recomendado)
- `wasm-pack`
- `cargo-generate` (opcional)
- `npm` o `yarn` para servir archivos estáticos

Instalación rápida:
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack build --target web

npx serve
O con Python:
python3 -m http.server 8080

abre con:
http://localhost:8080

📁 Configuración del Cargo.toml
Este proyecto utiliza las siguientes dependencias:

serde = { version = "1.0.0", features = ["derive"] }
web = { version = "0.21", features = ["csr"] }
web-sys = { version = "0.3.88", features = ["Event", "InputEvent"] }
wasm-bindgen = "0.2.111"
gloo = "0.6.0"
wasm-bindgen-futures = "0.4.62"
