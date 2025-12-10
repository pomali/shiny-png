#!/usr/bin/env python3
import http.server
import socketserver
import os

class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add MIME type for WASM
        if self.path.endswith('.wasm'):
            self.send_header('Content-Type', 'application/wasm')
        elif self.path.endswith('.js'):
            self.send_header('Content-Type', 'application/javascript')
        super().end_headers()

    def do_GET(self):
        # Add proper headers for WASM
        if self.path.endswith('.wasm'):
            self.send_response(200)
            self.send_header('Content-Type', 'application/wasm')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            try:
                with open(os.path.join(os.getcwd(), self.path.lstrip('/')), 'rb') as f:
                    self.wfile.write(f.read())
            except:
                self.send_error(404)
            return
        super().do_GET()

PORT = 8000
os.chdir(os.path.dirname(os.path.abspath(__file__)))

with socketserver.TCPServer(("", PORT), MyHTTPRequestHandler) as httpd:
    print(f"Serving at http://localhost:{PORT}")
    httpd.serve_forever()
