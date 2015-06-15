from flask import Flask
from flask_sockets import Sockets

app = Flask("venice-server")
sockets = Sockets(app)

@sockets.route('/echo')
def echo_socket(ws):
    while True:
        message = ws.receive()
        ws.send(message)

def run(**args):
    app.run(**args)
