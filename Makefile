.PHONY: all run clean

all: run

.env_setup:
	virtualenv --no-site-packages -p python2.7 env
	env/bin/pip install -U flask flask_sockets ipython gunicorn
	touch $@

run: .env_setup
	env/bin/gunicorn -k flask_sockets.worker server.server:app

clean:
	rm -f .env_setup
	rm -rf env/
	find . -name '*.pyc' -delete
