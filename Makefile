.PHONY: all

all: .env_setup

.env_setup:
	virtualenv --no-site-packages env
	env/bin/pip install -U flask ipython
	touch $@

clean:
	rm -f .env_setup
	rm -rf env/
