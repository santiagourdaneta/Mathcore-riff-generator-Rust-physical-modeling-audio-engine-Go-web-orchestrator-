# 🎸 Lazarus Engine v2.0

**Lazarus Engine** es un generador procedimental de riffs de **Mathcore** diseñado para explorar la intersección entre el modelado físico de audio y la narrativa distópica. 

Este proyecto utiliza un motor de audio en **Rust** para simular la vibración de cuerdas en afinación **Drop C** mediante el algoritmo *Karplus-Strong*, orquestado por una interfaz web ligera construida en **Go**.

## 🚀 Características
- **Afinación Drop C:** Frecuencias optimizadas entre 30Hz y 100Hz para un sonido pesado.
- **Modelado Físico:** Síntesis de cuerdas reales, no simples ondas senoidales.
- **Double-Bass Kick:** Bombo sincronizado automáticamente con los chugs de la guitarra.
- **Orquestador Go:** Interfaz web minimalista para control en tiempo real.
- **Lazarus Logs:** Generación automática de bitácoras literarias con cada riff.

## 🛠️ Tecnologías
- **Rust:** Procesamiento de señal (DSP) y generación de WAV.
- **Go:** Servidor web y automatización de procesos.
- **HTML5/JS:** Interfaz de usuario "Glitch-style".

## 📦 Instalación y Uso

1. **Requisitos:** Tener instalados Rust (Cargo) y Go.
2. **Clonar y Compilar:**
   ```bash
   git clone [https://github.com/tu-usuario/lazarus-engine.git](https://github.com/tu-usuario/lazarus-engine.git)
   cd lazarus-engine
   cargo build