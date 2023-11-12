import logging
import sys


class Colors:
    GREY = "\x1b[37m"
    GREEN = "\x1b[32m"
    YELLOW = "\x1b[33m"
    RED = "\x1b[31m"
    PURPLE = "\x1b[35m"
    BLUE = "\x1b[34m"
    LIGHT_BLUE = "\x1b[36m"
    BLINK_RED = "\x1b[5m\x1b[31m"
    RESET = "\x1b[0m"
    BOLD = "\033[1m"
    UNDERLINE = "\033[4m"
    END = "\033[0m"


class CustomFormatter(logging.Formatter):
    """Logging Formatter to add colors and count warning / errors"""

    def __init__(self):
        super(CustomFormatter, self).__init__()
        module = f"{Colors.BLUE}%(module)-6s{Colors.RESET} "
        lvl = f"{Colors.BOLD}%(levelname)+8s:{Colors.END} "
        msg = "%(message)s"
        self.FORMATS = {
            logging.DEBUG: module
            + Colors.PURPLE
            + lvl
            + Colors.PURPLE
            + msg
            + Colors.RESET,
            logging.INFO: module + Colors.GREY + lvl + Colors.GREY + msg + Colors.RESET,
            logging.WARNING: module
            + Colors.YELLOW
            + lvl
            + Colors.YELLOW
            + msg
            + Colors.RESET,
            logging.ERROR: module + Colors.RED + lvl + Colors.RED + msg + Colors.RESET,
            logging.CRITICAL: module
            + Colors.BLINK_RED
            + lvl
            + Colors.BLINK_RED
            + msg
            + Colors.RESET,
        }

    def format(self, record):
        log_fmt = self.FORMATS.get(record.levelno)
        formatter = logging.Formatter(log_fmt)
        return formatter.format(record)


console_handler = logging.StreamHandler(sys.stdout)
console_handler.setFormatter(CustomFormatter())
logging.basicConfig(
    datefmt="%Y-%m-%dT%H:%M:%S",
    handlers=[
        console_handler,
    ],
    level=logging.DEBUG,
)

log = logging.getLogger()
