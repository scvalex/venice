.PHONY: all run clean

all: run

.env_setup:
	virtualenv --no-site-packages env
	env/bin/pip install -U flask ipython
	touch $@

run: .env_setup
	( . env/bin/activate && python run.py )

clean:
	rm -f .env_setup
	rm -rf env/
