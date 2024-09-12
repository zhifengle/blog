import logging
from pathlib import Path

# Configure logging
class CustomFormatter(logging.Formatter):
    def formatTime(self, record, datefmt=None):
        return self.convertTime(record)

    def convertTime(self, record):
        return logging.Formatter.formatTime(self, record, datefmt="%Y-%m-%dT%H:%M:%SZ")

    def format(self, record):
        levelname = record.levelname.ljust(5)
        name = record.name
        # {record.filename}:{record.lineno}
        log_message = f"[{self.formatTime(record)} {levelname} {name}] {record.getMessage()}"
        return log_message


def get_logger(name):
    # logging.basicConfig(level=logging.DEBUG, filemode='a')
    # Create logger
    logger = logging.getLogger(name)
    logger.setLevel(logging.DEBUG)
    formatter = CustomFormatter()

    # logger.propagate = False
    log_dir = Path.home() / 'Documents/test/logs'
    # check if the directory exists
    if not log_dir.exists():
        log_dir.mkdir(parents=True)
    log_file = log_dir / f'{name}.log'
    # Set the formatter for the file handler
    file_handler = logging.FileHandler(log_file, encoding='utf-8')
    file_handler.setFormatter(formatter)
    logger.addHandler(file_handler)

    # Create a console handler
    console_handler = logging.StreamHandler()
    console_handler.setFormatter(formatter)
    logger.addHandler(console_handler)
    return logger