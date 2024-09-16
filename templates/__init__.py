"""
{name}
~~~~~~~~~~~~~~~~~~~

{desc}

:copyright: (c) {year} {author}
:license: {license}, see LICENSE for more details.

"""


__title__ = '{name}'
__author__ = '{author}'
__license__ = '{license}'
__copyright__ = 'Copyright {year} {name}'
__version__ = '0.0.1'

__path__ = __import__('pkgutil').extend_path(__path__, __name__)

import logging
from typing import NamedTuple, Literal


class VersionInfo(NamedTuple):
    major: int
    minor: int
    micro: int
    releaselevel: Literal["alpha", "beta", "candidate", "final"]
    serial: int


version_info: VersionInfo = VersionInfo(major=0, minor=0, micro=1, releaselevel='final', serial=0)

logging.getLogger(__name__).addHandler(logging.NullHandler())

del logging, NamedTuple, Literal, VersionInfo
