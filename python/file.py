import pathlib
def read_current_file() -> str:
    """
    read current script file first line and print
    """
    p = pathlib.Path(__file__).resolve()
    with p.open() as f:
        return f.readline()