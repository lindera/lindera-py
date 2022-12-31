# lindera-py

Python bindings for [indera](https://github.com/lindera-morphology/lindera), a Japanese morphological analysis engine.

## Usage

This library is experimental at this time and is not available on pypi.

If you want to use it, please build it on your interpreter environment using `maturin` as follows.

```shell
git clone https://github.com/lindera-morphology/lindera-py.git
maturin develop --release
```

See [here](https://github.com/PyO3/maturin) for how to install [maturin](https://github.com/PyO3/maturin).

## Config file

The specification of the configuration file is shared with [lindera]().
Please refer to [here](https://github.com/lindera-morphology/lindera/blob/main/resources/lindera_ipadic_conf.json) for configuration.
