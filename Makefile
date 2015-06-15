.PHONY: all

all: .env_setup

.env_setup:
	virtualenv --no-site-packages venv
	venv/bin/pip install -U flask ipython
	touch $@

clean:
	rm -f .env_setup
	rm -rf venv/
