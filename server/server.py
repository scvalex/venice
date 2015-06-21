from __future__ import print_function

from flask import Flask, send_from_directory
from flask_sockets import Sockets
import os

app = Flask("venice-server")
sockets = Sockets(app)

def send_from_root(d, f):
    return send_from_directory(os.path.join(os.getcwd(), 'server', d), f)

@sockets.route('/echo')
def echo_socket(ws):
    while True:
        message = ws.receive()
        print('echo:', message)
        ws.send(message)

@app.route('/')
def hello():
    return 'Hello World!'

@app.route('/console')
def console():
    return send_from_root('static', 'console.html')

@app.route('/js/<path:path>')
def send_js(path):
    return send_from_root('js', path)

@app.route('/css/<path:path>')
def send_css(path):
    return send_from_root('css', path)
