# nfl_sims
A play-by-play simulator for NFL games


## Python install on Mac OS X

First install `snappy` system-wide
`brew install snappy`

Download Python 3.11.6

Then create a python-3.11.6 virtual environment in root:
`python3.11 -m venv venv`

Source it:
`source venv/bin/activate`

Move to the `src/` directory, which holds all Python code:
`cd src/`

Then install poetry:
`pip install poetry`

If you are on an ARM Mac (e.g. M1), run:
`CPPFLAGS="-I/opt/homebrew/include -L/opt/homebrew/lib" poetry install --no-root`

If you are on Intel-based Mac, run:
`CPPFLAGS="-I/usr/local/include -L/usr/local/lib" poetry install --no-root`

The `CPPFLAGS` are necessary because Python needs to know where your `snappy` was installed
