package main

import (
	"fmt"
	"html/template"
	"net/http"
	"os"
	"os/exec"
)

func main() {
	http.HandleFunc("/", homeHandler)
	http.HandleFunc("/generate", generateHandler)
	fmt.Println("🎸 Lazarus Engine en http://localhost:8080")
	http.ListenAndServe(":8080", nil)
}

func homeHandler(w http.ResponseWriter, r *http.Request) {
	tmpl := `
	<!DOCTYPE html>
	<html>
	<head>
		<title>LAZARUS ENGINE</title>
		<style>
			body { background: #0a0a0a; color: #00ff41; font-family: monospace; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; }
			.container { border: 2px solid #00ff41; padding: 40px; border-radius: 15px; width: 450px; background: #111; text-align: center; box-shadow: 0 0 20px #00ff4144; }
			input { width: 100%; background: #1a1a1a; color: #00ff41; border: 1px solid #00ff41; padding: 12px; margin: 10px 0; outline: none; }
			button { width: 100%; background: #00ff41; color: #000; border: none; padding: 15px; cursor: pointer; font-weight: bold; margin-top: 10px; }
			#toast { visibility: hidden; min-width: 250px; background: #00ff41; color: #000; padding: 16px; position: fixed; bottom: 30px; font-weight: bold; border-radius: 5px; }
			#toast.show { visibility: visible; animation: fade 3s; }
			@keyframes fade { from {opacity:0} to {opacity:1} }
		</style>
	</head>
	<body>
		<div class="container">
			<h2>LAZARUS_GEN v2.0</h2>
			<form id="f">
				<label>BPM</label><input type="number" name="bpm" value="160">
				<label>GAIN</label><input type="number" name="gain" value="90">
				<label>BARS</label><input type="number" name="bars" value="4">
				<button type="submit">EXECUTE_GENERATION</button>
			</form>
		</div>
		<div id="toast">✅ RIFF Y BITÁCORA GENERADOS</div>
		<script>
			document.getElementById('f').onsubmit = async (e) => {
				e.preventDefault();
				await fetch('/generate', { method: 'POST', body: new FormData(e.target) });
				document.getElementById('toast').className = "show";
				setTimeout(() => { document.getElementById('toast').className = ""; }, 3000);
			};
		</script>
	</body>
	</html>`
	t, _ := template.New("w").Parse(tmpl)
	t.Execute(w, nil)
}

func generateHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodPost {
		bpm, gain, bars := r.FormValue("bpm"), r.FormValue("gain"), r.FormValue("bars")
		exec.Command("cargo", "run", "--", "-b", bpm, "-g", gain, "-n", bars).Run()

		log := fmt.Sprintf("Bitácora Lazarus\nBPM: %s\nLa piedra hecha de fe ha vibrado en Drop C.", bpm)
		os.WriteFile("lazarus_log.txt", []byte(log), 0644)
		w.WriteHeader(http.StatusOK)
	}
}
