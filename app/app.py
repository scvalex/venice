from flask import Flask

app = Flask("venice")

@app.route('/')
def hello_world():
    return 'Hello, World!'

def run(**args):
    app.run(**args)
